use std::process;
use std::sync::Arc;

use ami_core::cache::get_cover_art_cache_path;
use ami_core::config::Config;
use anyhow::{Context, Result};
use axum::Router;
use axum::routing::get;
use mpris_server::Server;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::sync::{RwLock, broadcast};
use tower_http::services::ServeDir;

use crate::command_handler::handle_command;
use crate::commands::Command;
use crate::daemon_process::PID_FILE;
use crate::internal_events::{InternalEvent, handle_internal_event};
use crate::orchestrator::Orchestrator;
use crate::services::mpris::Mpris;
use crate::services::{daemon_addr, daemon_addr_listen};
use crate::websockets::WebSocketService;

pub type SharedState = Arc<RwLock<Orchestrator>>;
pub type MprisServer = Arc<RwLock<Server<Mpris>>>;
const CHANNEL_CAPACITY: usize = 32;

pub struct App {
    pub orchestrator: SharedState,
    pub internal_event_rx: Option<UnboundedReceiver<InternalEvent>>,
    pub mpris_server: Option<MprisServer>,
    pub listen: bool,
    pub config: Config,
}

impl App {
    pub fn new(listen: bool) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel::<InternalEvent>();
        let config = Config::load()?;
        let orchestrator = Arc::new(RwLock::new(Orchestrator::new(tx, &config)?));

        Ok(App {
            orchestrator,
            mpris_server: None,
            internal_event_rx: Some(rx),
            listen,
            config,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        // Create PID file to prevent concurrent session.
        fs::write(PID_FILE, process::id().to_string()).await?;

        let (command_tx, command_rx) = mpsc::unbounded_channel::<Command>();

        let mpris = Mpris::new(self.orchestrator.clone(), command_tx);
        self.mpris_server = mpris.start().await.ok();

        let player = Arc::clone(&self.orchestrator.read().await.clone_player_arc());

        // A broadcast channel: one sender, many receivers (one per client)
        let (connection_tx, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
        let connection_tx = Arc::new(connection_tx); // Share the sender across tasks

        let tx = Arc::clone(&connection_tx);
        tokio::spawn(async move { Orchestrator::send_player_position(player, &tx).await });

        let ws_service = WebSocketService::new(
            connection_tx.clone(),
            self.orchestrator.clone(),
            self.mpris_server.clone(),
        );

        let internal_event_rx = self.internal_event_rx.take().context("Error: internal_event_rx was already taken.")?;
        Self::spawn_message_handler(internal_event_rx, connection_tx.clone(), command_rx, self.mpris_server.clone(), self.orchestrator.clone());

        let daemon_addr = if self.listen {
            daemon_addr_listen()?
        } else {
            daemon_addr()?
        };

        let listener = TcpListener::bind(daemon_addr.clone()).await?;
        let listen_msg = format!("Server listening on {daemon_addr}");
        log::debug!("{listen_msg}");
        println!("{listen_msg}");

        let router = Router::new()
            .route("/", get(WebSocketService::ws_handler))
            .fallback_service(ServeDir::new(get_cover_art_cache_path()?))
            .with_state(ws_service);

        axum::serve(listener, router).await?;

        Ok(())
    }

    fn spawn_message_handler(mut internal_event_rx: UnboundedReceiver<InternalEvent>, connection_tx: Arc<Sender<String>>, mut command_rx: UnboundedReceiver<Command>, mpris_server: Option<MprisServer>, shared_state: SharedState) {
        tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(event) = internal_event_rx.recv() => {
                            let mut state = shared_state.write().await;
                            let _ = handle_internal_event(
                                event,
                                &mut state,
                                &connection_tx.clone(),
                                mpris_server.clone(),
                            ).await.inspect_err(|e| log::error!("Internal event error: {e}"));
                        }
                        Some(cmd) = command_rx.recv() => {
                            let _ = handle_command(cmd, shared_state.clone(), &connection_tx.clone(), mpris_server.clone()).await.inspect_err(|e| log::error!("Command error: {e}"));
                        }
                    }
                }
            });
    }
}

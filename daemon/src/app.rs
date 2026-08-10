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
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::sync::{RwLock, broadcast};
use tower_http::services::ServeDir;

use crate::command_handler::handle_command;
use crate::commands::Command;
use crate::daemon_process::PID_FILE;
use crate::internal_events::{InternalEvent, handle_internal_event};
use crate::orchestrator::Orchestrator;
use crate::services::mpris::Mpris;
use crate::services::{daemon_addr};
use crate::websockets::WebSocketService;

pub type SharedState = Arc<RwLock<Orchestrator>>;
pub type MprisServer = Arc<RwLock<Server<Mpris>>>;
const CHANNEL_CAPACITY: usize = 32;

pub struct App {
    pub orchestrator: SharedState,
    pub internal_event_rx: Option<UnboundedReceiver<InternalEvent>>,
    pub mpris_server: Option<MprisServer>,
}

impl App {
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel::<InternalEvent>();
        Ok(App {
            orchestrator: Arc::new(RwLock::new(Orchestrator::new(tx)?)),
            mpris_server: None,
            internal_event_rx: Some(rx),
        })
    }

    pub async fn run(mut self) -> Result<()> {
        // Create PID file to prevent concurrent session.
        fs::write(PID_FILE, process::id().to_string()).await?;
        log::debug!("Daemon starting...");

        let config = Config::load()?;

        self.orchestrator
            .write()
            .await
            .load_library_config(config.library);

        let (command_tx, mut command_rx) = mpsc::unbounded_channel::<Command>();

        let mpris = Mpris::new(self.orchestrator.clone(), command_tx);
        self.mpris_server = mpris.start().await.ok();

        let player = Arc::clone(&self.orchestrator.read().await.clone_player_arc());

        let daemon_addr = daemon_addr()?;
        let listener = TcpListener::bind(daemon_addr.clone()).await?;
        log::debug!("Server listening on {daemon_addr}");

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

        let mut internal_event_rx = self.internal_event_rx.take().context("Error: internal_event_rx was already taken.")?;
        let shared_state = self.orchestrator.clone();
        tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(event) = internal_event_rx.recv() => {
                            let mut state = shared_state.write().await;
                            let _ = handle_internal_event(
                                event,
                                &mut state,
                                &connection_tx.clone(),
                                self.mpris_server.clone(),
                            ).await.inspect_err(|e| log::error!("Internal event error: {e}"));
                        }
                        Some(cmd) = command_rx.recv() => {
                            let _ = handle_command(cmd, shared_state.clone(), &connection_tx.clone(), self.mpris_server.clone()).await.inspect_err(|e| log::error!("Command error: {e}"));
                        }
                    }
                }
            });

        let router = Router::new()
            .route("/", get(WebSocketService::ws_handler))
            .fallback_service(ServeDir::new(get_cover_art_cache_path()?))
            .with_state(ws_service);

        axum::serve(listener, router).await?;

        Ok(())
    }
}

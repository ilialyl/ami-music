use std::process;
use std::sync::Arc;

use ami_core::config::Config;
use anyhow::Result;
use mpris_server::Server;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::sync::{RwLock, broadcast};

use crate::commands::Command;
use crate::daemon_process::PID_FILE;
use crate::internal_events::InternalEvent;
use crate::orchestrator::Orchestrator;
use crate::services;
use crate::services::mpris::Mpris;
use crate::websockets::WebSocketService;

pub type SharedState = Arc<RwLock<Orchestrator>>;
pub type MprisServer = Arc<RwLock<Server<Mpris>>>;
pub const DAEMON_ADDR: &str = "0.0.0.0:7878";
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

        let (command_tx, command_rx) = mpsc::unbounded_channel::<Command>();

        let mpris = Mpris::new(self.orchestrator.clone(), command_tx);
        self.mpris_server = mpris.start().await.ok();

        services::run_cover_art_service()?;

        let player = Arc::clone(&self.orchestrator.read().await.clone_player_arc());

        let listener = TcpListener::bind(DAEMON_ADDR).await?;
        log::debug!("Server listening on {DAEMON_ADDR}");

        // A broadcast channel: one sender, many receivers (one per client)
        let (connection_tx, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
        let connection_tx = Arc::new(connection_tx); // Share the sender across tasks

        let tx = Arc::clone(&connection_tx);
        tokio::spawn(async move { Orchestrator::send_player_position(player, &tx).await });

        let mut ws_service = WebSocketService::new(
            listener,
            connection_tx,
            self.internal_event_rx
                .take()
                .expect("internal_event_rx already taken"),
            command_rx,
            self.orchestrator.clone(),
            self.mpris_server.clone(),
        );
        ws_service.start().await
    }
}

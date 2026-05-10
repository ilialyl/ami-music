use std::process;
use std::sync::Arc;

use ami_core::config::Config;
use anyhow::{Context, Result};
use tokio::fs;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{RwLock, broadcast};

use crate::daemon_process::PID_FILE;
use crate::internal_events::InternalEvent;
use crate::orchestrator::Orchestrator;
use crate::services;
use crate::services::mpris::Mpris;
use crate::websockets::WebSocketService;

pub type SharedState = Arc<RwLock<Orchestrator>>;
pub const DAEMON_ADDR: &str = "0.0.0.0:7878";
const CHANNEL_CAPACITY: usize = 32;

pub struct App {
    pub orchestrator: SharedState,
    pub internal_event_rx: Option<UnboundedReceiver<InternalEvent>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<InternalEvent>();
        Ok(App {
            orchestrator: Arc::new(RwLock::new(Orchestrator::new(tx)?)),
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

        let mpris = Mpris::new(self.orchestrator.clone());
        let mpris_server = mpris.start();

        services::run_thumbnail_service()?;

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
                .context("Internal Event receiver went missing.")?,
            self.orchestrator.clone(),
        );
        ws_service.start().await
    }
}

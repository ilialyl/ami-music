use std::{sync::Arc};

use anyhow::Result;
use axum::{extract::{State, WebSocketUpgrade, ws::{Message, WebSocket}}, response::IntoResponse};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{broadcast},
};

use crate::{
    app::{MprisServer, SharedState}, command_handler::handle_command, commands::Command,
};

#[derive(Clone)]
pub struct WebSocketService {
    pub connection_tx: Arc<broadcast::Sender<String>>,
    pub shared_state: SharedState,
    pub mpris_server: Option<MprisServer>,
}

impl WebSocketService {
    pub fn new(
        connection_tx: Arc<broadcast::Sender<String>>,
        shared_state: SharedState,
        mpris_server: Option<MprisServer>,
    ) -> Self {
        Self {
            connection_tx,
            shared_state,
            mpris_server,
        }
    }

    pub async fn ws_handler(
        ws: WebSocketUpgrade,
        State(state): State<WebSocketService>
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| async move {
                if let Err(e) = WebSocketService::handle_connection(
                    socket,
                    state.connection_tx,
                    state.shared_state,
                    state.mpris_server,
                ).await {
                    log::error!("Connection error: {e}");
                }
        })
    }

    pub async fn handle_connection(
        ws: WebSocket,
        connection_tx: Arc<broadcast::Sender<String>>,
        shared_state: SharedState,
        mpris_server: Option<MprisServer>,
    ) -> Result<()> {
        // Split the WebSocket into a writer (sink) and reader (stream)
        let (mut ws_sink, mut ws_stream) = ws.split();

        // Subscribe to the broadcast channel to receive messages from other clients
        let mut connection_rx = connection_tx.subscribe();

        loop {
            tokio::select! {
                // Receive messages from clients.
                msg = ws_stream.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Ok(cmd) = serde_json::from_str::<Command>(&text) {
                                log::debug!("Received a message from client: {}", text);
                                {
                                    // Handle commands. Mutate state and send messages to the local broadcast channel if needed.
                                    handle_command(cmd, shared_state.clone(), &connection_tx, mpris_server.clone()).await?;
                                }
                        }}
                        // Client disconnected or error
                        _ => break,
                    }
                }

                // Send messages accumulated in the local broadcast channel to all clients.
                broadcast = connection_rx.recv() => {
                    match broadcast {
                        Ok(text) => {
                            if ws_sink.send(Message::Text(text.into())).await.is_err() {
                                break;
                        }}
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            log::warn!("lagged, dropped {n} messages");
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
        Ok(())
    }
}

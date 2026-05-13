use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc::UnboundedReceiver},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{
    app::{MprisServer, SharedState},
    command_handler::handle_command,
    commands::Command,
    internal_events::{InternalEvent, handle_internal_event},
};

pub struct WebSocketService {
    pub listener: TcpListener,
    pub connection_tx: Arc<broadcast::Sender<String>>,
    pub internal_event_rx: Option<UnboundedReceiver<InternalEvent>>,
    pub command_rx: Option<UnboundedReceiver<Command>>,
    pub shared_state: SharedState,
    pub mpris_server: Option<MprisServer>,
}

impl WebSocketService {
    pub fn new(
        listener: TcpListener,
        connection_tx: Arc<broadcast::Sender<String>>,
        internal_event_rx: UnboundedReceiver<InternalEvent>,
        command_rx: UnboundedReceiver<Command>,
        shared_state: SharedState,
        mpris_server: Option<MprisServer>,
    ) -> Self {
        Self {
            listener,
            connection_tx,
            internal_event_rx: Some(internal_event_rx),
            command_rx: Some(command_rx),
            shared_state,
            mpris_server,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        let mut internal_event_rx = self.internal_event_rx.take().unwrap();
        let mut command_rx = self.command_rx.take().unwrap();
        let connection_tx = Arc::clone(&self.connection_tx);
        let shared_state = self.shared_state.clone();
        let mpris_server = self.mpris_server.clone();
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

        loop {
            let (stream, peer) = self.listener.accept().await?;

            // Spawn a new async task for each client — they run concurrently
            tokio::spawn(Self::handle_connection(
                stream,
                peer,
                self.connection_tx.clone(),
                self.shared_state.clone(),
                self.mpris_server.clone(),
            ));
        }
    }

    pub async fn handle_connection(
        stream: TcpStream,
        peer: SocketAddr,
        connection_tx: Arc<broadcast::Sender<String>>,
        shared_state: SharedState,
        mpris_server: Option<MprisServer>,
    ) -> Result<()> {
        log::debug!("{peer} connected");

        // Upgrade the TCP connection to a WebSocket
        let ws = accept_async(stream).await.unwrap();

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
                            log::warn!("{peer} lagged, dropped {n} messages");
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }

        log::debug!("{peer} disconnected");

        Ok(())
    }
}

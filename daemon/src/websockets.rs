use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{
    command_handler::handle_command,
    commands::Command,
    internal_events::{InternalEvent, handle_internal_event},
    states::SharedState,
};

pub struct WebSocketService {
    pub listener: TcpListener,
    pub connection_tx: Arc<broadcast::Sender<String>>,
    pub internal_event_rx: Option<tokio::sync::mpsc::UnboundedReceiver<InternalEvent>>,
    pub shared_state: SharedState,
}

impl WebSocketService {
    pub fn new(
        listener: TcpListener,
        connection_tx: Arc<broadcast::Sender<String>>,
        internal_event_rx: tokio::sync::mpsc::UnboundedReceiver<InternalEvent>,
        shared_state: SharedState,
    ) -> Self {
        Self {
            listener,
            connection_tx,
            internal_event_rx: Some(internal_event_rx),
            shared_state,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        let mut internal_event_rx = self.internal_event_rx.take().unwrap();
        let connection_tx = Arc::clone(&self.connection_tx);
        let shared_state = self.shared_state.clone();
        tokio::spawn(async move {
            loop {
                if let Some(event) = internal_event_rx.recv().await {
                    let mut state = shared_state.write().await;
                    if let Err(e) =
                        handle_internal_event(event, &mut state, &connection_tx.clone()).await
                    {
                        log::error!("Internal event error: {e}");
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
            ));
        }
    }

    pub async fn handle_connection(
        stream: TcpStream,
        peer: SocketAddr,
        connection_tx: Arc<broadcast::Sender<String>>,
        shared_state: SharedState,
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
                                    handle_command(cmd, shared_state.clone(), &connection_tx).await?;
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

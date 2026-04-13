use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use snowy_core::config::Config;
use snowy_daemon::{
    commands::Command,
    handler::handle_command,
    internal_events::{InternalEvent, handle_internal_event},
    orchestrator::Orchestrator,
    states::AppState,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, broadcast},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

// How many messages the broadcast channel can buffer
const CHANNEL_CAPACITY: usize = 32;

#[tokio::main]
async fn main() -> Result<()> {
    let (internal_event_tx, _) = broadcast::channel::<InternalEvent>(CHANNEL_CAPACITY);
    let internal_event_tx = Arc::new(internal_event_tx);

    let state = Arc::new(Mutex::new(AppState::new(internal_event_tx.clone())?));
    let config = Config::load()?;
    state.lock().await.library.load(config.library);

    let player = state.lock().await.orchestrator.playback.player.clone();

    tokio::spawn(Orchestrator::watch_track_end(
        player,
        internal_event_tx.clone(),
    ));

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {addr}");

    // A broadcast channel: one sender, many receivers (one per client)
    let (connection_tx, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let connection_tx = Arc::new(connection_tx); // Share the sender across tasks

    loop {
        let (stream, peer) = listener.accept().await.unwrap();
        let connection_tx = Arc::clone(&connection_tx);
        let internal_event_rx = Arc::clone(&internal_event_tx);

        // Spawn a new async task for each client — they run concurrently
        tokio::spawn(handle_connection(
            stream,
            peer,
            connection_tx,
            internal_event_rx,
            state.clone(),
        ));
    }
}

async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    connection_tx: Arc<broadcast::Sender<String>>,
    internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
    state: Arc<Mutex<AppState>>,
) -> Result<()> {
    println!("{peer} connected");

    // Upgrade the TCP connection to a WebSocket
    let ws = accept_async(stream).await.unwrap();

    // Split the WebSocket into a writer (sink) and reader (stream)
    let (mut ws_sink, mut ws_stream) = ws.split();

    // Subscribe to the broadcast channel to receive messages from other clients
    let mut connection_rx = connection_tx.subscribe();
    let mut internal_event_rx = internal_event_tx.subscribe();

    loop {
        tokio::select! {
            // Receive messages from clients.
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<Command>(&text) {
                            let mut state = state.lock().await;
                            // Handle commands. Mutate state and send messages to the local broadcast channel if needed.
                            handle_command(cmd, &mut state, &connection_tx).await?;
                    }}
                    // Client disconnected or error
                    _ => break,
                }
            }

            // Send messages accumulated in the local broadcast channel to all clients.
            broadcast = connection_rx.recv() => {
                if let Ok(text) = broadcast {
                    let _ = ws_sink.send(Message::Text(text.into())).await;
                }
            }

            internal_event = internal_event_rx.recv() => {
                if let Ok(event) = internal_event {
                    let mut state = state.lock().await;
                    handle_internal_event(event, &mut state, &connection_tx).await?;
                }
            }
        }
    }

    println!("{peer} disconnected");

    Ok(())
}

use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use snowy_core::config::Config;
use snowy_daemon::{commands::Command, handler::handle_command, states::AppState};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, broadcast},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

// How many messages the broadcast channel can buffer
const CHANNEL_CAPACITY: usize = 32;

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(Mutex::new(AppState::new()?));
    let config = Config::load()?;
    state.lock().await.library.load(config.library);

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {addr}");

    // A broadcast channel: one sender, many receivers (one per client)
    let (tx, _rx) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let tx = Arc::new(tx); // Share the sender across tasks

    loop {
        let (stream, peer) = listener.accept().await.unwrap();
        let tx = Arc::clone(&tx);

        // Spawn a new async task for each client — they run concurrently
        tokio::spawn(handle_connection(stream, peer, tx, state.clone()));
    }
}

async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    tx: Arc<broadcast::Sender<String>>,
    state: Arc<Mutex<AppState>>,
) -> Result<()> {
    println!("{peer} connected");

    // Upgrade the TCP connection to a WebSocket
    let ws = accept_async(stream).await.unwrap();

    // Split the WebSocket into a writer (sink) and reader (stream)
    let (mut ws_sink, mut ws_stream) = ws.split();

    // Subscribe to the broadcast channel to receive messages from other clients
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            // A message arrived from THIS client → broadcast to everyone
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<Command>(&text) {
                            let mut state = state.lock().await;
                            // Mutate state based on command
                            handle_command(cmd, &mut state, &tx).await?;
                    }}
                    // Client disconnected or error
                    _ => break,
                }
            }

            // A broadcast message arrived → forward it to THIS client
            broadcast = rx.recv() => {
                if let Ok(text) = broadcast {
                    let _ = ws_sink.send(Message::Text(text.into())).await;
                }
            }
        }
    }

    println!("{peer} disconnected");

    Ok(())
}

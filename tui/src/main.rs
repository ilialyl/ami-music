use ami_daemon::{
    commands::{Command, LibraryCommand},
    events::ServerEvent,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc::{self};

use crate::app::App;

pub mod app;
pub mod event;
pub mod state;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

async fn connect() {
    let url = "ws://127.0.0.1:8080";
    let (ws, _) = tokio_tungstenite::connect_async(url).await.unwrap();
    println!("Connected to {url}");

    let (mut ws_sink, mut ws_stream) = ws.split();

    // 1. Create an unbounded channel to communicate from the REPL (blocking) to WebSocket (async)
    // We pass your custom `Command` enum through this channel.
    let (tx, mut rx) = mpsc::unbounded_channel::<Command>();

    // Initial fetch command
    let json = serde_json::to_string(&Command::Library(LibraryCommand::Fetch)).unwrap();
    ws_sink
        .send(tokio_tungstenite::tungstenite::Message::Text(json.into()))
        .await
        .unwrap();

    loop {
        tokio::select! {
            // User typed a command in easy-repl → receive it from channel, send to the server
            cmd_opt = rx.recv() => {
                match cmd_opt {
                    Some(cmd) => {
                        let json = serde_json::to_string(&cmd).unwrap();
                        ws_sink.send(tokio_tungstenite::tungstenite::Message::Text(json.into())).await.unwrap();
                    }
                    None => break, // Channel closed (REPL thread finished, user quit)
                }
            }

            // Server sent something → print it
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                        if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                            match event {
                                ServerEvent::SendLibrary(tracks) => println!("{tracks:#?}"),
                                ServerEvent::SendQueue(queue) => println!("{queue:#?}"),
                                ServerEvent::SendPlayerSnapshot(snapshot) => println!("{snapshot:#?}"),
                                _ => {},
                            }
                        }
                    },
                    Some(Err(e)) => {
                        println!("WebSocket error: {}", e);
                        break;
                    }
                    None => break, // Connection closed
                    _ => {} // Ignore binary, pings, etc.
                }
            }
        }
    }
}

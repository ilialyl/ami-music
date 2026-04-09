use futures_util::{SinkExt, StreamExt};
use snowy_daemon::commands::{Command, LibraryCommand, QueueCommand};
use snowy_daemon::events::ServerEvent;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080";
    let (ws, _) = connect_async(url).await.unwrap();
    println!("Connected to {url}");

    let (mut ws_sink, mut ws_stream) = ws.split();

    // Read lines from stdin in a background task
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    let json = serde_json::to_string(&Command::Library(LibraryCommand::Fetch)).unwrap();
    ws_sink.send(Message::Text(json.into())).await.unwrap();

    loop {
        tokio::select! {
            // User typed something → send it to the server
            line = stdin.next_line() => {
                match line.unwrap() {
                    Some(_) => {
                        let json = serde_json::to_string(&Command::Queue(QueueCommand::Clear)).unwrap();
                        ws_sink.send(Message::Text(json.into())).await.unwrap();
                    }
                    None => break, // stdin closed (Ctrl+D)
                }
            }

            // Server sent something → print it
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) =>
                    {
                        if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                        match event {
                            ServerEvent::SendLibrary(tracks) => println!("{tracks:#?}"),
                            ServerEvent::SendQueue(queue) => println!("{queue:#?}"),
                        }}
                    },
                    _ => break,
                }
            }
        }
    }
}

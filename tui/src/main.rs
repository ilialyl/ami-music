use std::{path::PathBuf, sync::Arc, time::SystemTime};

use ami_core::{library::TrackId, track::Track};
use ami_daemon::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
};
use color_eyre::eyre::Result;
use futures::{SinkExt, StreamExt};
use ratatui_image::picker::Picker;
use tokio::{
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver},
    },
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::{app::App, state::DaemonStates, ui::cover_art::CoverArt};

pub mod app;
pub mod event;
pub mod handler;
pub mod state;
pub mod ui;

const DAEMON_URL: &str = "ws://0.0.0.0:7878";
const COVER_URL: &str = "http://0.0.0.0:7879";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    setup_logger()?;

    match tokio_tungstenite::connect_async(DAEMON_URL).await {
        Ok((ws, _)) => {
            log::debug!("Connected to {DAEMON_URL}");

            let states = Arc::new(Mutex::new(DaemonStates::default()));
            let terminal = ratatui::init();

            let (tx, rx) = mpsc::unbounded_channel::<Command>();

            let app = App::new(states.clone(), tx);

            tokio::spawn(connect(ws, rx, states, app.image_picker.clone()));
            let result = app.run(terminal).await;
            result
        }
        Err(e) => {
            eprintln!("Error connecting to {}.\n[{}]", DAEMON_URL, e);
            Ok(())
        }
    }
}

async fn connect(
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut rx: UnboundedReceiver<Command>,
    states: Arc<Mutex<DaemonStates>>,
    image_picker: Arc<Picker>,
) -> Result<()> {
    let (mut ws_sink, mut ws_stream) = ws.split();

    // Initial fetch commands
    let commands = vec![
        Command::Library(LibraryCommand::Fetch),
        Command::Queue(QueueCommand::Fetch),
        Command::Playback(PlaybackCommand::GetSnapshot),
    ];
    let jsons: Vec<String> = commands
        .iter()
        .filter_map(|c| serde_json::to_string(c).ok())
        .collect();

    for j in &jsons {
        ws_sink
            .send(tokio_tungstenite::tungstenite::Message::Text(j.into()))
            .await?;
    }

    loop {
        tokio::select! {
            cmd_opt = rx.recv() => {
                match cmd_opt {
                    Some(cmd) => {
                        let json = serde_json::to_string(&cmd).unwrap();
                        ws_sink.send(tokio_tungstenite::tungstenite::Message::Text(json.into())).await?;
                    }
                    None => break, // Channel closed
                }
            }

            // Mutate States
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                        if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                            match event {
                                ServerEvent::SendLibrary(tracks) => {
                                    let mut library: Vec<(TrackId, Arc<Track>)> = tracks
                                                .iter()
                                                .map(|(&k, v)| (k, v.clone()))
                                                .collect();
                                    library.sort_by(|(_, a), (_, b)| a.metadata.title.cmp(&b.metadata.title));

                                    states.lock().await.library_snapshot = library;
                                },
                                ServerEvent::SendQueue(queue) => {
                                    if let Some(current_track) = queue.current_track.as_ref() {
                                            // Get and parse cover art if cover cache path exists
                                            let id = current_track.id.clone();
                                            let mut locked_states = states.lock().await;
                                            let cloned_states = states.clone();
                                            let already_loaded: bool = matches!(locked_states.cover_art, Some(ref cover_art) if cover_art.0 == id);
                                            if !already_loaded {
                                                if let Some(thumb_path) = current_track.metadata.cover_art_path.as_ref() {
                                                    if let Some(filename) = thumb_path.file_name().and_then(|s| s.to_str()) {
                                                        let url = Url::parse(&format!("{}/{}", COVER_URL, filename))?;
                                                        let picker = image_picker.clone();
                                                        tokio::spawn(async move {
                                                            if let Ok(Some(protocol)) = CoverArt::parse_cover_art(url, picker).await {
                                                                cloned_states.lock().await.cover_art = Some((id, protocol));
                                                            }
                                                        });
                                                    }
                                                } else {
                                                    locked_states.cover_art = None;
                                                }
                                            }
                                        }
                                        states.lock().await.queue_snapshot = queue;
                                },
                                ServerEvent::SendPlayerSnapshot(snapshot) => {
                                    states.lock().await.player_snapshot = snapshot;
                                },

                                ServerEvent::SendPlayerPosition(duration) => {
                                    states.lock().await.player_snapshot.position = duration;
                                }
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

    Ok(())
}

pub fn setup_logger() -> Result<()> {
    let log_path = PathBuf::from("/home/lyns0/projects/personal/ami/tui.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("lofty", log::LevelFilter::Error)
        .level_for("zbus", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

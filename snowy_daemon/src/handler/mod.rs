use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
    states::AppState,
};

pub fn handle_command(
    command: Command,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        Command::Playback(cmd) => handle_playback_command(cmd, state, tx),
        Command::Queue(cmd) => handle_queue_command(cmd, state, tx),
        Command::Library(cmd) => handle_library_command(cmd, state, tx),
    }
}

pub fn handle_playback_command(
    command: PlaybackCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        PlaybackCommand::Play => todo!(),
        PlaybackCommand::Pause => todo!(),
        PlaybackCommand::TogglePlay => todo!(),
        PlaybackCommand::SetPosition(_) => todo!(),
        PlaybackCommand::Seek { .. } => todo!(),
        PlaybackCommand::Restart => todo!(),
        PlaybackCommand::IncreaseVol { .. } => todo!(),
        PlaybackCommand::DecreaseVol { .. } => todo!(),
        PlaybackCommand::SetVolume { .. } => todo!(),
    }
}

pub fn handle_queue_command(
    command: QueueCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { .. } => todo!(),
        QueueCommand::Prepend { .. } => todo!(),
        QueueCommand::Dequeue { .. } => todo!(),
        QueueCommand::Next => todo!(),
        QueueCommand::Prev => todo!(),
        QueueCommand::Shuffle => todo!(),
        QueueCommand::Clear => todo!(),
    }
}

pub fn handle_library_command(
    command: LibraryCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event = ServerEvent::SendLibrary(state.library.tracks.clone());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}

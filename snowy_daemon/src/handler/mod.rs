use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
    states::AppState,
};

pub async fn handle_command(
    command: Command,
    state: &mut AppState,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        Command::Playback(cmd) => handle_playback_command(cmd, state, connection_tx).await,
        Command::Queue(cmd) => handle_queue_command(cmd, state, connection_tx).await,
        Command::Library(cmd) => handle_library_command(cmd, state, connection_tx).await,
    }
}

pub async fn handle_playback_command(
    command: PlaybackCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        PlaybackCommand::Play => state.orchestrator.playback.play().await,

        PlaybackCommand::Pause => state.orchestrator.playback.pause().await,

        PlaybackCommand::TogglePlay => state.orchestrator.playback.toggle_play(),

        PlaybackCommand::SetPosition(pos) => state.orchestrator.playback.set_position(pos)?,

        PlaybackCommand::Seek { offset_seconds } => {
            state.orchestrator.playback.seek(offset_seconds)?
        }

        PlaybackCommand::Restart => state.orchestrator.playback.seek(0)?,

        PlaybackCommand::IncreaseVol { step } => state.orchestrator.playback.increase_volume(step),

        PlaybackCommand::DecreaseVol { step } => state.orchestrator.playback.decrease_volume(step),

        PlaybackCommand::SetVolume { value } => state.orchestrator.playback.set_volume(value),
    };

    let event = ServerEvent::SendPlayerSnapshot(state.orchestrator.playback.get_snapshot());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_queue_command(
    command: QueueCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { track_id } => state.orchestrator.enqueue(track_id),
        QueueCommand::Prepend { track_id } => state.orchestrator.prepend(track_id),
        QueueCommand::Dequeue { index } => state.orchestrator.dequeue(index),
        QueueCommand::Next => state.orchestrator.next(),
        QueueCommand::Prev => state.orchestrator.prev(),
        QueueCommand::Shuffle => state.orchestrator.shuffle(),
        QueueCommand::Clear => state.orchestrator.clear(),
        QueueCommand::Fetch => {}
    };

    let event = ServerEvent::SendQueue(state.orchestrator.queue.clone());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_library_command(
    command: LibraryCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event = ServerEvent::SendLibrary(state.orchestrator.library.tracks.clone());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}

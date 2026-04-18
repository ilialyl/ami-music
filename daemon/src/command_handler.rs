use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{Mutex, broadcast};

use crate::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
    states::AppState,
};

pub async fn handle_command(
    command: Command,
    state: Arc<Mutex<AppState>>,
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
    state: Arc<Mutex<AppState>>,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        PlaybackCommand::Play => state.lock().await.orchestrator.playback.play(),

        PlaybackCommand::Pause => state.lock().await.orchestrator.playback.pause(),

        PlaybackCommand::TogglePlay => state.lock().await.orchestrator.playback.toggle_play(),

        PlaybackCommand::SetPosition(pos) => {
            state.lock().await.orchestrator.playback.set_position(pos)?
        }

        PlaybackCommand::Seek { offset_seconds } => state
            .lock()
            .await
            .orchestrator
            .playback
            .seek(offset_seconds)?,

        PlaybackCommand::Restart => state.lock().await.orchestrator.playback.seek(0)?,

        PlaybackCommand::IncreaseVol { step } => state
            .lock()
            .await
            .orchestrator
            .playback
            .increase_volume(step),

        PlaybackCommand::DecreaseVol { step } => state
            .lock()
            .await
            .orchestrator
            .playback
            .decrease_volume(step),

        PlaybackCommand::SetVolume { value } => {
            state.lock().await.orchestrator.playback.set_volume(value)
        }
        PlaybackCommand::GetSnapshot => {}
    };

    let event =
        ServerEvent::SendPlayerSnapshot(state.lock().await.orchestrator.playback.get_snapshot());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_queue_command(
    command: QueueCommand,
    state: Arc<Mutex<AppState>>,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { track_id } => {
            state.lock().await.orchestrator.enqueue(track_id).await?
        }
        QueueCommand::Prepend { track_id } => state.lock().await.orchestrator.prepend(track_id),
        QueueCommand::Dequeue { index } => state.lock().await.orchestrator.dequeue(index),
        QueueCommand::Next => state.lock().await.orchestrator.next().await?,
        QueueCommand::Prev => state.lock().await.orchestrator.prev().await?,
        QueueCommand::Shuffle => state.lock().await.orchestrator.shuffle(),
        QueueCommand::Clear => state.lock().await.orchestrator.clear(),
        QueueCommand::Fetch => {}
    };

    let event = ServerEvent::SendQueue(state.lock().await.orchestrator.queue.clone());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_library_command(
    command: LibraryCommand,
    state: Arc<Mutex<AppState>>,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event =
                ServerEvent::SendLibrary(state.lock().await.orchestrator.library.tracks.clone());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}

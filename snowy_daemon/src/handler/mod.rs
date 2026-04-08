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
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        Command::Playback(cmd) => handle_playback_command(cmd, state, tx).await,
        Command::Queue(cmd) => handle_queue_command(cmd, state, tx).await,
        Command::Library(cmd) => handle_library_command(cmd, state, tx).await,
    }
}

pub async fn handle_playback_command(
    command: PlaybackCommand,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        PlaybackCommand::Play => state.orchestrator.lock().await.playback.play(),

        PlaybackCommand::Pause => state.orchestrator.lock().await.playback.pause(),

        PlaybackCommand::TogglePlay => state.orchestrator.lock().await.playback.toggle_play(),

        PlaybackCommand::SetPosition(pos) => {
            state.orchestrator.lock().await.playback.set_position(pos)?
        }

        PlaybackCommand::Seek { offset_seconds } => state
            .orchestrator
            .lock()
            .await
            .playback
            .seek(offset_seconds)?,

        PlaybackCommand::Restart => state.orchestrator.lock().await.playback.seek(0)?,

        PlaybackCommand::IncreaseVol { step } => state
            .orchestrator
            .lock()
            .await
            .playback
            .increase_volume(step),

        PlaybackCommand::DecreaseVol { step } => state
            .orchestrator
            .lock()
            .await
            .playback
            .decrease_volume(step),

        PlaybackCommand::SetVolume { value } => {
            state.orchestrator.lock().await.playback.set_volume(value)
        }
    };

    Ok(())
}

pub async fn handle_queue_command(
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
    };

    Ok(())
}

pub async fn handle_library_command(
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

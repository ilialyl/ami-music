use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
    states::SharedState,
};

pub async fn handle_command(
    command: Command,
    state: SharedState,
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
    state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    let no_broadcast = matches!(command, PlaybackCommand::Seek { .. });

    match command {
        PlaybackCommand::Play => state.read().await.orchestrator.playback.play(),

        PlaybackCommand::Pause => state.read().await.orchestrator.playback.pause(),

        PlaybackCommand::TogglePlay => state.read().await.orchestrator.playback.toggle_play(),

        PlaybackCommand::SetPosition(pos) => {
            state.read().await.orchestrator.playback.set_position(pos)?
        }

        PlaybackCommand::Seek { offset_seconds } => state
            .read()
            .await
            .orchestrator
            .playback
            .seek(offset_seconds)?,

        PlaybackCommand::Restart => state.read().await.orchestrator.playback.seek(0)?,

        PlaybackCommand::IncreaseVol { step } => state
            .read()
            .await
            .orchestrator
            .playback
            .increase_volume(step),

        PlaybackCommand::DecreaseVol { step } => state
            .read()
            .await
            .orchestrator
            .playback
            .decrease_volume(step),

        PlaybackCommand::SetVolume { value } => {
            state.read().await.orchestrator.playback.set_volume(value)
        }

        PlaybackCommand::GetSnapshot => {}
    };

    if !no_broadcast {
        let event = ServerEvent::SendPlayerSnapshot(
            state.read().await.orchestrator.playback.get_snapshot(),
        );
        let json = serde_json::to_string(&event)?;
        let _ = tx.send(json);
    }

    Ok(())
}

pub async fn handle_queue_command(
    command: QueueCommand,
    state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { track_id } => {
            let mut state = state.write().await;
            state.orchestrator.enqueue(track_id).await?;
            if state.orchestrator.playback.player.empty() {
                state.orchestrator.next().await?;
            }
        }
        QueueCommand::Prepend { track_id } => state.write().await.orchestrator.prepend(track_id),
        QueueCommand::Dequeue { index } => state.write().await.orchestrator.dequeue(index),
        QueueCommand::PlayNow { track_id } => {
            let mut state = state.write().await;
            state.orchestrator.prepend(track_id);
            state.orchestrator.next().await?;
        }
        QueueCommand::Next => {
            state.write().await.orchestrator.next().await?;
        }
        QueueCommand::Prev => state.write().await.orchestrator.prev().await?,
        QueueCommand::Shuffle => state.write().await.orchestrator.shuffle(),
        QueueCommand::Clear => state.write().await.orchestrator.clear(),
        QueueCommand::SetLoopMode(mode) => state.write().await.orchestrator.queue.loop_mode = mode,
        QueueCommand::CycleLoopMode => state.write().await.orchestrator.queue.cycle_loop_mode(),
        QueueCommand::Fetch => {}
    };

    let event = ServerEvent::SendQueue(state.read().await.orchestrator.queue.clone());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_library_command(
    command: LibraryCommand,
    state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event =
                ServerEvent::SendLibrary(state.read().await.orchestrator.library.tracks.clone());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}

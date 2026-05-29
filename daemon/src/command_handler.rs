use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    app::{MprisServer, SharedState},
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
};

pub async fn handle_command(
    command: Command,
    state: SharedState,
    connection_tx: &broadcast::Sender<String>,
    mpris_server: Option<MprisServer>,
) -> Result<()> {
    match command {
        Command::Playback(cmd) => {
            handle_playback_command(cmd, state, connection_tx, &mpris_server).await
        }
        Command::Queue(cmd) => handle_queue_command(cmd, state, connection_tx, &mpris_server).await,
        Command::Library(cmd) => {
            handle_library_command(cmd, state, connection_tx, &mpris_server).await
        }
    }
}

pub async fn handle_playback_command(
    command: PlaybackCommand,
    shared_state: SharedState,
    tx: &broadcast::Sender<String>,
    mpris_server: &Option<MprisServer>,
) -> Result<()> {
    let no_broadcast = matches!(command, PlaybackCommand::Seek { .. });

    match command {
        PlaybackCommand::Play => shared_state.read().await.play(mpris_server).await?,

        PlaybackCommand::Pause => shared_state.read().await.pause(mpris_server).await?,

        PlaybackCommand::TogglePlay => shared_state.read().await.toggle_play(mpris_server).await?,

        PlaybackCommand::SetPosition(pos) => {
            shared_state
                .read()
                .await
                .set_position(pos, mpris_server)
                .await?
        }

        PlaybackCommand::Seek { offset_seconds } => {
            shared_state
                .read()
                .await
                .seek(offset_seconds, mpris_server)
                .await?
        }

        PlaybackCommand::Restart => shared_state.read().await.rewind(mpris_server).await?,

        PlaybackCommand::IncreaseVol { step } => {
            shared_state
                .read()
                .await
                .increase_volume(step, mpris_server)
                .await?
        }

        PlaybackCommand::DecreaseVol { step } => {
            shared_state
                .read()
                .await
                .decrease_volume(step, mpris_server)
                .await?
        }

        PlaybackCommand::SetVolume { value } => {
            shared_state
                .read()
                .await
                .set_volume(value, mpris_server)
                .await?
        }

        PlaybackCommand::GetSnapshot => {}
    };

    if !no_broadcast {
        let event =
            ServerEvent::SendPlayerSnapshot(shared_state.read().await.get_player_snapshot());
        let json = serde_json::to_string(&event)?;
        let _ = tx.send(json);
    }

    Ok(())
}

pub async fn handle_queue_command(
    command: QueueCommand,
    shared_state: SharedState,
    tx: &broadcast::Sender<String>,
    mpris_server: &Option<MprisServer>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { track_id } => {
            let mut orchestrator = shared_state.write().await;
            orchestrator.enqueue(track_id, mpris_server).await?;
            let event = ServerEvent::SendPlayerSnapshot(orchestrator.get_player_snapshot());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
        QueueCommand::Prepend { track_id } => {
            shared_state
                .write()
                .await
                .prepend(track_id, mpris_server)
                .await?
        }
        QueueCommand::Dequeue { index } => {
            shared_state
                .write()
                .await
                .dequeue(index, mpris_server)
                .await?
        }
        QueueCommand::PlayNow { track_id } => {
            shared_state
                .write()
                .await
                .play_now(track_id, mpris_server)
                .await?
        }
        QueueCommand::Next => {
            shared_state.write().await.next(mpris_server).await?;
        }
        QueueCommand::Prev => shared_state.write().await.prev(mpris_server).await?,
        QueueCommand::Shuffle => shared_state.write().await.shuffle(),
        QueueCommand::Clear => shared_state.write().await.clear(),
        QueueCommand::SetLoopMode(loop_mode) => {
            shared_state
                .write()
                .await
                .set_loop_mode(loop_mode, mpris_server)
                .await?
        }
        QueueCommand::CycleLoopMode => {
            shared_state
                .write()
                .await
                .cycle_loop_mode(mpris_server)
                .await?
        }
        QueueCommand::Fetch => {}
    };

    let event = ServerEvent::SendQueue(shared_state.read().await.get_queue_snapshot());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_library_command(
    command: LibraryCommand,
    state: SharedState,
    tx: &broadcast::Sender<String>,
    _mpris_server: &Option<MprisServer>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event = ServerEvent::SendLibrary(state.read().await.get_library_snapshot());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}

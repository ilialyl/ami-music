use crate::{events::ServerEvent, states::AppState};
use ami_core::player::pause_reason::PauseReason;
use anyhow::Result;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum InternalEvent {
    PlayerEmpty,
}

pub async fn handle_internal_event(
    event: InternalEvent,
    state: &mut AppState,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match event {
        InternalEvent::PlayerEmpty => {
            if state.orchestrator.queue.next()
                && let Some(track) = state.orchestrator.queue.current_track.clone()
            {
                state
                    .orchestrator
                    .playback
                    .load_track(&track.pathbuf)
                    .await?;

                let event =
                    ServerEvent::SendPlayerSnapshot(state.orchestrator.playback.get_snapshot());
                let _ = connection_tx.send(serde_json::to_string(&event)?);
            } else if state.orchestrator.queue.current_track.is_some()
                && *state.orchestrator.playback.pause_reason.lock().await != PauseReason::Exhaustion
            {
                state.orchestrator.playback.on_exhaustion().await;
                let event =
                    ServerEvent::SendPlayerSnapshot(state.orchestrator.playback.get_snapshot());
                let _ = connection_tx.send(serde_json::to_string(&event)?);
            }
        }
    }

    Ok(())
}

use crate::{events::ServerEvent, states::AppState};
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
                state.orchestrator.playback.load_track(&track.pathbuf)?;

                let event =
                    ServerEvent::SendPlayerSnapshot(state.orchestrator.playback.get_snapshot());
                let _ = connection_tx.send(serde_json::to_string(&event)?);
            }
        }
    }

    Ok(())
}

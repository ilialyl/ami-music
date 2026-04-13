use crate::states::AppState;
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
            print!("player empty");
            if !state.orchestrator.queue.is_empty() {
                state.orchestrator.queue.next();
                if let Some(current) = state.orchestrator.queue.current_track.clone() {
                    state.orchestrator.playback.load_track(&current.pathbuf)?;
                }
            } else {
                state.orchestrator.playback.pause();
            }
        }
    }

    Ok(())
}

use crate::{events::ServerEvent, states::AppState};
use ami_core::queue::loop_mode::LoopMode;
use anyhow::Result;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum InternalEvent {
    TrackEnded,
    SendPlayerPosition,
}

pub async fn handle_internal_event(
    event: InternalEvent,
    state: &mut AppState,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match event {
        InternalEvent::TrackEnded => {
            log::debug!("{:?} Received", event);
            match state.orchestrator.queue.loop_mode {
                LoopMode::None => {
                    if state.orchestrator.next().await? {
                        let events = [
                            ServerEvent::SendPlayerSnapshot(
                                state.orchestrator.playback.get_snapshot(),
                            ),
                            ServerEvent::SendQueue(state.orchestrator.queue.clone()),
                        ];
                        for e in events {
                            let _ = connection_tx.send(serde_json::to_string(&e)?);
                        }
                    }
                }
                LoopMode::Track => state.orchestrator.rewind()?,
                LoopMode::Queue => {
                    if state.orchestrator.queue.current_track.is_some() {
                        state.orchestrator.queue.restart();
                        if state.orchestrator.next().await? {
                            let events = [
                                ServerEvent::SendPlayerSnapshot(
                                    state.orchestrator.playback.get_snapshot(),
                                ),
                                ServerEvent::SendQueue(state.orchestrator.queue.clone()),
                            ];
                            for e in events {
                                let _ = connection_tx.send(serde_json::to_string(&e)?);
                            }
                        }
                    }
                }
            }
        }

        InternalEvent::SendPlayerPosition => {
            let event =
                ServerEvent::SendPlayerPosition(state.orchestrator.playback.player.get_pos());
            let _ = connection_tx.send(serde_json::to_string(&event)?);
        }
    }

    Ok(())
}

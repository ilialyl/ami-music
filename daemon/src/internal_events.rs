use crate::{events::ServerEvent, orchestrator::Orchestrator};
use ami_core::queue::loop_mode::LoopMode;
use anyhow::Result;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum InternalEvent {
    TrackEnded,
}

pub async fn handle_internal_event(
    event: InternalEvent,
    orchestrator: &mut Orchestrator,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match event {
        InternalEvent::TrackEnded => {
            log::debug!("{:?} Received", event);
            match orchestrator.queue.loop_mode {
                LoopMode::None => {
                    if orchestrator.next().await? {
                        let events = [
                            ServerEvent::SendPlayerSnapshot(orchestrator.playback.get_snapshot()),
                            ServerEvent::SendQueue(orchestrator.queue.clone()),
                        ];
                        for e in events {
                            let _ = connection_tx.send(serde_json::to_string(&e)?);
                        }
                    }
                }
                LoopMode::Track => orchestrator.rewind()?,
                LoopMode::Queue => {
                    if orchestrator.queue.current_track.is_some() {
                        orchestrator.queue.restart();
                        if orchestrator.next().await? {
                            let events = [
                                ServerEvent::SendPlayerSnapshot(
                                    orchestrator.playback.get_snapshot(),
                                ),
                                ServerEvent::SendQueue(orchestrator.queue.clone()),
                            ];
                            for e in events {
                                let _ = connection_tx.send(serde_json::to_string(&e)?);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

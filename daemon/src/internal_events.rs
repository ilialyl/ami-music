use crate::{app::MprisServer, events::ServerEvent, orchestrator::Orchestrator};
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
    mpris_server: Option<MprisServer>,
) -> Result<()> {
    match event {
        InternalEvent::TrackEnded => {
            log::debug!("{:?} Received", event);
            match orchestrator.loop_mode() {
                LoopMode::None => {
                    if orchestrator.next(&mpris_server).await? {
                        let events = [
                            ServerEvent::SendPlayerSnapshot(orchestrator.get_player_snapshot()),
                            ServerEvent::SendQueue(orchestrator.clone_queue()),
                        ];
                        for e in events {
                            let _ = connection_tx.send(serde_json::to_string(&e)?);
                        }
                    }
                }
                LoopMode::Track => orchestrator.rewind()?,
                LoopMode::Queue => {
                    if orchestrator.current_track().is_some() {
                        orchestrator.restart_queue();
                        if orchestrator.next(&mpris_server).await? {
                            let events = [
                                ServerEvent::SendPlayerSnapshot(orchestrator.get_player_snapshot()),
                                ServerEvent::SendQueue(orchestrator.clone_queue()),
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

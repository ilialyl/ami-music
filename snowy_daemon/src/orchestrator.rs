use std::sync::Arc;

use anyhow::Result;
use rodio::Player;
use snowy_core::{player::Playback, queue::Queue};
use tokio::sync::broadcast;

use crate::internal_events::InternalEvent;

pub struct Orchestrator {
    pub playback: Arc<Playback>,
    pub queue: Queue,
    pub internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
}

impl Orchestrator {
    pub fn new(tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(Orchestrator {
            playback: Arc::new(Playback::new()?),
            queue: Queue::default(),
            internal_event_tx: tx,
        })
    }

    pub async fn watch_track_end(
        player: Arc<Player>,
        internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
    ) {
        loop {
            let p = Arc::clone(&player);
            tokio::task::spawn_blocking(move || p.sleep_until_end())
                .await
                .unwrap();

            let _ = internal_event_tx.send(InternalEvent::PlayerEmpty);
        }
    }
}

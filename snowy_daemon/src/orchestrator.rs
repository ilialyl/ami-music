use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rodio::Player;
use snowy_core::{
    library::{Library, TrackId},
    player::Playback,
    queue::Queue,
};
use tokio::sync::broadcast;

use crate::internal_events::InternalEvent;

pub struct Orchestrator {
    pub playback: Arc<Playback>,
    pub queue: Queue,
    pub library: Library,
    pub internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
}

impl Orchestrator {
    pub fn new(tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(Orchestrator {
            playback: Arc::new(Playback::new()?),
            queue: Queue::default(),
            library: Library::default(),
            internal_event_tx: tx,
        })
    }

    pub async fn watch_track_end(
        player: Arc<Player>,
        internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
    ) {
        loop {
            if player.empty() {
                tokio::time::sleep(Duration::from_millis(200)).await;
                continue;
            }

            let p = Arc::clone(&player);
            tokio::task::spawn_blocking(move || p.sleep_until_end())
                .await
                .unwrap();

            let _ = internal_event_tx.send(InternalEvent::PlayerEmpty);
            log::debug!("Sent PlayerEmpty");
        }
    }

    pub fn enqueue(&mut self, id: TrackId) {
        if let Some(track) = self.library.tracks.get(&id) {
            self.queue.enqueue(track.clone())
        }
    }

    pub fn prepend(&mut self, id: TrackId) {
        if let Some(track) = self.library.tracks.get(&id) {
            self.queue.prepend_queue(track.clone())
        }
    }

    pub fn dequeue(&mut self, index: usize) {
        self.queue.dequeue(index);
    }

    pub fn next(&mut self) {
        self.queue.next();
    }

    pub fn prev(&mut self) {
        self.queue.prev();
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle();
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

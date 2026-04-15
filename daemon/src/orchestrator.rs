use std::{sync::Arc, time::Duration};

use ami_core::{
    library::{Library, TrackId},
    player::{Playback, pause_reason::PauseReason},
    queue::Queue,
};
use anyhow::Result;
use rodio::Player;
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

    pub async fn enqueue(&mut self, id: TrackId) -> Result<()> {
        if let Some(track) = self.library.tracks.get(&id) {
            self.queue.enqueue(track.clone());
            if self.queue.current_track.is_none()
                && self.queue.next()
                && let Some(track) = self.queue.current_track.as_ref()
            {
                self.playback.load_track(&track.pathbuf).await?;
                if *self.playback.pause_reason.lock().await != PauseReason::User {
                    self.playback.play().await;
                };
            }
        }

        Ok(())
    }

    pub fn prepend(&mut self, id: TrackId) {
        if let Some(track) = self.library.tracks.get(&id) {
            self.queue.prepend_queue(track.clone())
        }
    }

    pub fn dequeue(&mut self, index: usize) {
        self.queue.dequeue(index);
    }

    pub async fn next(&mut self) -> Result<()> {
        if self.queue.next()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.load_track(&track.pathbuf).await?;
            if *self.playback.pause_reason.lock().await != PauseReason::User {
                self.playback.play().await;
            };
        }

        Ok(())
    }

    pub async fn prev(&mut self) -> Result<()> {
        if self.queue.prev()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.load_track(&track.pathbuf).await?;
            if *self.playback.pause_reason.lock().await != PauseReason::User {
                self.playback.play().await;
            };
        }

        Ok(())
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle();
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

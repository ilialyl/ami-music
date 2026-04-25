use std::{path::Path, sync::Arc, thread::sleep, time::Duration};

use ami_core::{
    library::{Library, TrackId},
    player::Playback,
    queue::Queue,
};
use anyhow::Result;
use rodio::{Player, source::EmptyCallback};
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
        println!("Loading Playback...");
        let playback = Arc::new(Playback::new()?);
        println!("Loading Queue...");
        let queue = Queue::default();
        println!("Loading Library...");
        let library = Library::default();
        Ok(Orchestrator {
            playback,
            queue,
            library,
            internal_event_tx: tx,
        })
    }

    fn load_track(&self, audio_path: &Path) -> Result<()> {
        let tx = self.internal_event_tx.clone();
        self.playback.load_track(audio_path)?;
        self.playback
            .player
            .append(EmptyCallback::new(Box::new(move || {
                let _ = tx.send(InternalEvent::TrackEnded);
            })));

        Ok(())
    }

    pub fn watch_track_end(
        player: Arc<Player>,
        internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
    ) {
        loop {
            let p = Arc::clone(&player);

            p.sleep_until_end();

            sleep(Duration::from_secs(3));

            let _ = internal_event_tx.send(InternalEvent::TrackEnded);
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
                self.load_track(&track.pathbuf)?;
                self.playback.play();
            } else if self.queue.current_track.is_some() && self.playback.player.empty() {
                self.load_track(&track.pathbuf)?;
                self.playback.play();
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

    pub async fn next(&mut self) -> Result<bool> {
        if self.queue.next()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
            self.playback.play();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn prev(&mut self) -> Result<()> {
        if self.queue.prev()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
            self.playback.play();
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

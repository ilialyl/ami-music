use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use ami_core::{
    library::{Library, TrackId},
    player::Playback,
    queue::Queue,
};
use anyhow::Result;
use rodio::{Player, source::EmptyCallback};
use tokio::sync::{broadcast, mpsc::UnboundedSender};

use crate::{events::ServerEvent, internal_events::InternalEvent};

pub struct Orchestrator {
    pub playback: Arc<Playback>,
    pub queue: Queue,
    pub library: Library,
    pub internal_event_tx: UnboundedSender<InternalEvent>,
}

impl Orchestrator {
    pub fn new(tx: UnboundedSender<InternalEvent>) -> Result<Self> {
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
        let fired = Arc::new(AtomicBool::new(false));
        self.playback
            .player
            .append(EmptyCallback::new(Box::new(move || {
                if fired
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    let _ = tx.send(InternalEvent::TrackEnded);
                }
            })));

        Ok(())
    }

    pub fn rewind(&self) -> Result<()> {
        if let Some(track) = self.queue.current_track.as_ref() {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
        }

        Ok(())
    }

    pub async fn send_player_position(
        player: Arc<Player>,
        connection_tx: &broadcast::Sender<String>,
    ) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            if !player.is_paused() {
                let event = ServerEvent::SendPlayerPosition(player.get_pos());
                let _ = connection_tx.send(serde_json::to_string(&event)?);
            }
        }
    }

    pub async fn enqueue(&mut self, id: TrackId) -> Result<()> {
        if let Some(track) = self.library.tracks.get(&id).cloned() {
            self.queue.enqueue(track.clone());
            log::debug!("Called Orchestrator::enqueue");
            if self.queue.current_track.is_none() {
                self.next().await?;
            } else if self.queue.current_track.is_some() && self.playback.player.empty() {
                self.next().await?;
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

use anyhow::Result;

use crate::{player::Playback, queue::Queue};

pub struct Orchestrator {
    pub playback: Playback,
    pub queue: Queue,
}

impl Orchestrator {
    pub fn new() -> Result<Self> {
        Ok(Orchestrator {
            playback: Playback::new()?,
            queue: Queue::default(),
        })
    }

    pub fn play(&mut self) -> Result<()> {
        if let Some(current) = self.queue.current_track.as_mut() {
            self.playback.load_track(&current.pathbuf)?;
            self.playback.play();
        }

        Ok(())
    }
}

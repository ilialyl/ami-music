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
}

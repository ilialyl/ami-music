use anyhow::Result;

use crate::{player::Playback, queue::Queue};

pub struct Orchestrator {
    pub player: Playback,
    pub queue: Queue,
}

impl Orchestrator {
    pub fn new() -> Result<Self> {
        Ok(Orchestrator {
            player: Playback::new()?,
            queue: Queue::default(),
        })
    }
}

use crate::{player::Playback, queue::Queue};

pub struct Orchestrator {
    player: Playback,
    queue: Queue,
}

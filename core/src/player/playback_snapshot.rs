use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::player::playback_status::PlaybackStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSnapshot {
    pub playback_status: PlaybackStatus,
    pub volume: f32,
    pub playback_speed: f32,
    pub position: Duration,
}

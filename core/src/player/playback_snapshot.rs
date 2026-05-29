use std::time::Duration;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::player::playback_status::PlaybackStatus;

#[derive(Default, Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "player_snapshot.ts")]
pub struct PlayerSnapshot {
    pub playback_status: PlaybackStatus,
    pub volume: f32,
    pub playback_speed: f32,
    #[ts(type = "{ secs: number, nanos: number }")]
    pub position: Duration,
}

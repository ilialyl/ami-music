use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
#[ts(export, export_to = "playback.ts")]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Stopped,
}

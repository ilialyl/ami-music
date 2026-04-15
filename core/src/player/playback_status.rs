use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

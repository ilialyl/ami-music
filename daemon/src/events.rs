use std::{collections::HashMap, time::Duration};

use ami_core::{
    library::TrackId, player::playback_snapshot::PlayerSnapshot,
    queue::queue_snapshot::QueueSnapshot, track::Track,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Track>),
    SendQueue(QueueSnapshot),
    SendPlayerSnapshot(PlayerSnapshot),
    SendPlayerPosition(Duration),
}

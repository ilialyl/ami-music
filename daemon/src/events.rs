use std::{collections::HashMap, sync::Arc};

use ami_core::{
    library::TrackId, player::playback_snapshot::PlayerSnapshot, queue::Queue, track::Track,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Arc<Track>>),
    SendQueue(Queue),
    SendPlayerSnapshot(PlayerSnapshot),
}

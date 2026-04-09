use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use snowy_core::{library::TrackId, queue::Queue, track::Track};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Arc<Track>>),
    SendQueue(Queue),
}

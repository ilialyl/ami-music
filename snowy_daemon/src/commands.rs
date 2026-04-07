use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "cmd")]
pub enum Command {
    Enqueue { track_id: u64 },
    Prepend { track_id: u64 },
    Dequeue { position: usize, track_id: u64 },
    GetLibrary,
}

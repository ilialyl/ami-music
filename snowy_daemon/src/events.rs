use serde::{Deserialize, Serialize};
use snowy_core::track::Track;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(Vec<Track>),
}

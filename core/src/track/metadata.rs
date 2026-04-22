use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub length: u128,
    pub album: Option<String>,
    pub title: String,
    pub artist: Option<String>,
    pub disc_number: Option<u32>,
    pub genre: Option<String>,
    pub year: Option<u32>,
}

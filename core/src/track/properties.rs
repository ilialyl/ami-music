use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Properties {
    pub bitrate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub channels: Option<u8>,
    pub sample_rate: Option<u32>,
    pub duration: Duration,
}

use std::time::Duration;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export, export_to = "properties.ts")]
pub struct Properties {
    pub bitrate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub channels: Option<u8>,
    pub sample_rate: Option<u32>,
    #[ts(type = "{ secs: number, nanos: number }")]
    pub duration: Duration,
}

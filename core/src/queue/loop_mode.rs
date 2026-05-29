use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, TS)]
#[ts(export, export_to = "loop_mode.ts")]
pub enum LoopMode {
    #[default]
    None,
    Track,
    Queue,
}

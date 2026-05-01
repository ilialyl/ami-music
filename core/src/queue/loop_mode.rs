use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub enum LoopMode {
    #[default]
    None,
    Track,
    Queue,
}

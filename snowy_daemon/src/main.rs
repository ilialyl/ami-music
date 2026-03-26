use std::sync::Arc;

use anyhow::Result;
use snowy_core::orchestrator::Orchestrator;
use tokio::sync::broadcast;

use crate::state::AppState;

pub mod sockets;
pub mod state;

fn main() -> Result<()> {
    println!("Hello, world!");
    let (tx, _rx) = broadcast::channel(16);
    let app_state = Arc::new(AppState {
        orchestrator: Orchestrator::new()?,
        broadcast_tx: tx,
    });

    Ok(())
}

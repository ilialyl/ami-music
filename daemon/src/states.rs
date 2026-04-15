use std::sync::Arc;

use anyhow::Result;
use tokio::sync::broadcast;

use crate::internal_events::InternalEvent;
use crate::orchestrator::Orchestrator;

pub struct AppState {
    pub orchestrator: Orchestrator,
}

impl AppState {
    pub fn new(internal_event_tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(AppState {
            orchestrator: Orchestrator::new(internal_event_tx)?,
        })
    }
}

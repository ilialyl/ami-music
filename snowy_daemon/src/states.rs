use std::sync::Arc;

use anyhow::Result;
use snowy_core::library::Library;
use tokio::sync::Mutex;

use crate::orchestrator::Orchestrator;

pub struct AppState {
    pub orchestrator: Arc<Mutex<Orchestrator>>,
    pub library: Library,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(AppState {
            orchestrator: Arc::new(Mutex::new(Orchestrator::new()?)),
            library: Library::default(),
        })
    }
}

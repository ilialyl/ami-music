use snowy_core::orchestrator::Orchestrator;
use tokio::sync::broadcast;

pub struct AppState {
    pub orchestrator: Orchestrator,
    pub broadcast_tx: broadcast::Sender<String>,
}

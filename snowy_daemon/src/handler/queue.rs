use snowy_core::library::TrackId;

use crate::states::AppState;

pub async fn enqueue(id: TrackId, state: &mut AppState) {
    if let Some(track) = state.library.tracks.get(&id) {
        state.orchestrator.lock().await.queue.enqueue(track.clone())
    }
}

use std::{fmt::Debug, sync::Arc};

use ami_core::{
    library::TrackId, player::playback_snapshot::PlayerSnapshot, queue::Queue, track::Track,
};
use ratatui_image::protocol::StatefulProtocol;

#[derive(Default)]
pub struct DaemonStates {
    pub player_snapshot: PlayerSnapshot,
    pub queue_snapshot: Queue,
    pub library_snapshot: Vec<(TrackId, Arc<Track>)>,
    pub library_selected_index: usize,
    pub cover_art: Option<(TrackId, StatefulProtocol)>,
}

impl Debug for DaemonStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStates")
            .field("player_snapshot", &self.player_snapshot)
            .field("queue_snapshot", &self.queue_snapshot)
            .field("library_snapshot", &self.library_snapshot)
            .field("library_selected_index", &self.library_selected_index)
            .field(
                "cover_art",
                &self.cover_art.as_ref().map(|_| "<StatefulProtocol>"),
            )
            .finish()
    }
}

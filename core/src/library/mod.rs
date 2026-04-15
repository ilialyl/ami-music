use std::{collections::HashMap, fs::read_dir, str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{config::LibraryConfig, library::helper::is_rodio_supported, track::Track};

pub mod helper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TrackId(u64);

impl TrackId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTrackIdError;

impl FromStr for TrackId {
    type Err = ParseTrackIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TrackId(s.parse::<u64>().map_err(|_| ParseTrackIdError)?))
    }
}

#[derive(Default)]
pub struct Library {
    pub tracks: HashMap<TrackId, Arc<Track>>,
}

impl Library {
    pub fn load(&mut self, config: LibraryConfig) {
        self.tracks.clear();

        let mut id = TrackId::default();

        let track_vec: Vec<Track> = config
            .directories
            .iter()
            .filter(|path| path.is_dir())
            .flat_map(|dir| read_dir(dir).into_iter().flatten())
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter_map(|path| match is_rodio_supported(&path) {
                Ok(true) => Some(path),
                _ => None,
            })
            .filter_map(|path| Track::new(path.as_path()).ok())
            .collect();

        let mut tracks = HashMap::new();
        track_vec.into_iter().for_each(|t| {
            id.increment();
            tracks.insert(id, Arc::new(t));
        });

        self.tracks = tracks;
    }
}

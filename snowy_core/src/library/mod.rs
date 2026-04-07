use std::fs::read_dir;

use crate::{config::LibraryConfig, library::helper::is_rodio_supported, track::Track};

pub mod helper;

#[derive(Default)]
pub struct Library {
    pub tracks: Vec<Track>,
}

impl Library {
    pub fn load(&mut self, config: LibraryConfig) {
        self.tracks.clear();

        let mut id = 0;

        self.tracks = config
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
            .filter_map(|path| {
                id += 1;
                Track::new(path.as_path(), id).ok()
            })
            .collect();
    }
}

use std::{collections::VecDeque, sync::Arc};

use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::track::Track;

#[cfg(test)]
pub mod tests;

/// Struct to act as a queue of tracks.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Queue {
    pub current_track: Option<Arc<Track>>,
    pub previous_tracks: Vec<Arc<Track>>,
    pub next_tracks: VecDeque<Arc<Track>>,
}

impl Queue {
    /// Skip to the next track and push current track to the previous-track stack.
    /// Return boolean indicating whether the value changed or not.
    pub fn next(&mut self) -> bool {
        log::debug!("Called next()");
        if !self.next_tracks.is_empty()
            && let Some(next) = self.next_tracks.pop_front()
        {
            if let Some(curr) = self.current_track.take() {
                self.previous_tracks.push(curr);
            }
            self.current_track = Some(next);
            log::debug!("Skipped to next track");
            true
        } else {
            false
        }
    }

    /// Go back to the previous track and push-front current track to next-track queue.
    /// Return boolean indicating whether the value changed or not.
    pub fn prev(&mut self) -> bool {
        log::debug!("Called prev()");
        if !self.previous_tracks.is_empty()
            && let Some(prev) = self.previous_tracks.pop()
        {
            if let Some(curr) = self.current_track.take() {
                self.next_tracks.push_front(curr);
            }
            self.current_track = Some(prev);
            log::debug!("Skipped to previous track");
            true
        } else {
            false
        }
    }

    /// Enqueue a new track to the next-track queue.
    pub fn enqueue(&mut self, track: Arc<Track>) {
        log::debug!("Enqueuing {:?}", track.pathbuf);
        self.next_tracks.push_back(track);
    }

    /// Push-front a new track to the next-track queue.
    pub fn prepend_queue(&mut self, track: Arc<Track>) {
        log::debug!("Prepending {:?}", track.pathbuf);
        self.next_tracks.push_front(track);
    }

    pub fn dequeue(&mut self, index: usize) -> Option<Arc<Track>> {
        log::debug!("Removing index {index}");
        self.next_tracks.remove(index)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        let mut vec: Vec<Arc<Track>> = self.next_tracks.clone().into_iter().collect();
        vec.shuffle(&mut rng);
        self.next_tracks = vec.into_iter().collect();
        log::debug!("Shuffled queue");
    }

    pub fn clear(&mut self) {
        self.previous_tracks.clear();
        self.next_tracks.clear();

        log::debug!("Cleared queue");
    }

    pub fn is_empty(&self) -> bool {
        self.next_tracks.is_empty()
    }
}

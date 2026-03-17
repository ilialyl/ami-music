use std::collections::VecDeque;

use crate::track::Track;

#[cfg(test)]
pub mod tests;

/// Struct to act as a queue of tracks.
#[derive(Default)]
pub struct Queue {
    current_track: Option<Track>,
    previous_tracks: Vec<Track>,
    next_tracks: VecDeque<Track>,
}

impl Queue {
    /// Skip to the next track and push current track to the previous-track stack.
    pub fn next(&mut self) {
        if !self.next_tracks.is_empty()
            && let Some(next) = self.next_tracks.pop_front()
        {
            if let Some(curr) = self.current_track.take() {
                self.previous_tracks.push(curr);
            }
            self.current_track = Some(next);
        }
    }

    /// Go back to the previous track and push-front current track to next-track queue.
    pub fn prev(&mut self) {
        if !self.previous_tracks.is_empty()
            && let Some(prev) = self.previous_tracks.pop()
        {
            if let Some(curr) = self.current_track.take() {
                self.next_tracks.push_front(curr);
            }
            self.current_track = Some(prev);
        }
    }

    /// Enqueue a new track to the next-track queue.
    pub fn enqueue(&mut self, track: Track) {
        self.next_tracks.push_back(track);
    }

    /// Push-front a new track to the next-track queue.
    pub fn prepend_queue(&mut self, track: Track) {
        self.next_tracks.push_front(track);
    }
}

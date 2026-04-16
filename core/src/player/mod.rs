use std::{fs::File, path::Path, sync::Arc, time::Duration};

use anyhow::Result;
use rodio::{Decoder, MixerDeviceSink, Player};

use crate::player::{playback_snapshot::PlayerSnapshot, playback_status::PlaybackStatus};

#[cfg(test)]
pub mod tests;

pub mod mpris;
pub mod playback_snapshot;
pub mod playback_status;

/// Performs player-related functionalities.
pub struct Playback {
    pub player: Arc<Player>,
    _sink: MixerDeviceSink,
}

impl Playback {
    pub fn new() -> Result<Self> {
        let sink = rodio::DeviceSinkBuilder::open_default_sink()?;
        Ok(Playback {
            player: Arc::new(rodio::Player::connect_new(sink.mixer())),
            _sink: sink,
        })
    }

    /// Append audio source from path to the sink.
    pub fn load_track(&self, audio_path: &Path) -> Result<()> {
        log::debug!("Opening {audio_path:?}.");
        let source = Decoder::try_from(File::open(audio_path)?)?;
        self.player.append(source);

        Ok(())
    }

    pub async fn play(&self) {
        self.player.play();
    }

    pub async fn pause(&self) {
        self.player.pause();
    }

    pub fn toggle_play(&self) {
        if self.player.is_paused() {
            self.player.play();
            log::debug!("Set to playing");
        } else {
            self.player.pause();
            log::debug!("Set to paused");
        }
    }

    pub fn skip(&self) {
        log::debug!("Player Skipping one.");
        self.player.skip_one();
    }

    /// Returns f64 as volume
    pub fn volume(&self) -> f32 {
        self.player.volume()
    }

    pub fn increase_volume(&self, step: f32) {
        self.player.set_volume((self.volume() + step).min(2f32));
        log::debug!("Set volume to {}", self.volume());
    }

    pub fn decrease_volume(&self, step: f32) {
        self.player.set_volume((self.volume() - step).max(0f32));
        log::debug!("Set volume to {}", self.volume());
    }

    pub fn set_volume(&self, value: f32) {
        self.player.set_volume(value);
        log::debug!("Set volume to {}", self.volume());
    }

    pub fn playback_speed(&self) -> f32 {
        self.player.speed()
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        if self.player.empty() {
            PlaybackStatus::Stopped
        } else if self.player.is_paused() {
            PlaybackStatus::Paused
        } else {
            PlaybackStatus::Playing
        }
    }

    pub fn set_position(&self, pos: Duration) -> Result<()> {
        self.player.try_seek(pos)?;
        log::debug!("Set position to {:?}", self.player.get_pos());

        Ok(())
    }

    pub fn seek(&self, offset_seconds: i64) -> Result<()> {
        let duration = (self.player.get_pos().as_secs() as i64)
            .saturating_add(offset_seconds)
            .max(0) as u64;

        self.player.try_seek(Duration::from_secs(duration))?;
        log::debug!("Set position to {:?}", self.player.get_pos());

        Ok(())
    }

    pub fn get_snapshot(&self) -> PlayerSnapshot {
        PlayerSnapshot {
            playback_status: self.playback_status(),
            volume: self.volume(),
            playback_speed: self.playback_speed(),
            position: self.player.get_pos(),
        }
    }
}

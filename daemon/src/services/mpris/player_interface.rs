use ami_core::{player::playback_status, queue::loop_mode};
use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Time, TrackId, Volume,
    zbus::{self, fdo},
};
use url::Url;

use crate::services::{COVER_ADDR, mpris::Mpris};

impl PlayerInterface for Mpris {
    async fn next(&self) -> fdo::Result<()> {
        let mut orchestrator = self.shared_state.write().await;
        let _ = orchestrator
            .next()
            .await
            .inspect_err(|e| log::error!("{e}"));
        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        let mut orchestrator = self.shared_state.write().await;
        let _ = orchestrator
            .prev()
            .await
            .inspect_err(|e| log::error!("{e}"));
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        let orchestrator = self.shared_state.read().await;
        let _ = orchestrator.pause();
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        let orchestrator = self.shared_state.read().await;
        let _ = orchestrator.toggle_play();
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        let orchestrator = self.shared_state.read().await;
        let _ = orchestrator.play();
        Ok(())
    }

    async fn seek(&self, offset: Time) -> fdo::Result<()> {
        let orchestrator = self.shared_state.read().await;
        let _ = orchestrator.seek(offset.as_secs() as i64);
        Ok(())
    }

    async fn set_position(&self, _track_id: TrackId, position: Time) -> fdo::Result<()> {
        let orchestrator = self.shared_state.read().await;
        let _ = orchestrator.seek(position.as_secs() as i64);
        Ok(())
    }
    async fn position(&self) -> fdo::Result<Time> {
        let orchestrator = self.shared_state.read().await;

        Ok(Time::from_millis(
            orchestrator.player_position().as_millis() as i64,
        ))
    }

    async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        let orchestrator = self.shared_state.read().await;
        let status = match orchestrator.playback_status() {
            playback_status::PlaybackStatus::Paused => PlaybackStatus::Paused,
            playback_status::PlaybackStatus::Playing => PlaybackStatus::Playing,
            playback_status::PlaybackStatus::Stopped => PlaybackStatus::Stopped,
        };
        Ok(status)
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        let orchestrator = self.shared_state.read().await;
        let status = match orchestrator.loop_mode() {
            loop_mode::LoopMode::None => LoopStatus::None,
            loop_mode::LoopMode::Queue => LoopStatus::Playlist,
            loop_mode::LoopMode::Track => LoopStatus::Track,
        };
        Ok(status)
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) -> zbus::Result<()> {
        let loop_mode = match loop_status {
            LoopStatus::None => loop_mode::LoopMode::None,
            LoopStatus::Playlist => loop_mode::LoopMode::Queue,
            LoopStatus::Track => loop_mode::LoopMode::Track,
        };
        let mut orchestrator = self.shared_state.write().await;
        orchestrator.set_loop_mode(loop_mode);

        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        let orchestrator = self.shared_state.read().await;

        Ok(orchestrator.playback_speed() as f64)
    }

    async fn set_rate(&self, _rate: PlaybackRate) -> zbus::Result<()> {
        Ok(())
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        Ok(false)
    }

    async fn set_shuffle(&self, _shuffle: bool) -> zbus::Result<()> {
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        let orchestrator = self.shared_state.read().await;
        let metadata = if let Some(track) = orchestrator.get_current_track() {
            let mut m = Metadata::new();
            m.set_title(Some(track.metadata.title.clone()));
            m.set_album(track.metadata.album.clone());
            m.set_artist(track.metadata.artist.clone().map(|s| vec![s]));
            m.set_art_url(
                track
                    .pathbuf
                    .file_name()
                    .and_then(|s| s.to_str())
                    .and_then(|name| Url::parse(&format!("{}/{}", COVER_ADDR, name)).ok()),
            );
            m.set_disc_number(track.metadata.disc_number.map(|n| n as i32));
            m.set_genre(track.metadata.genre.clone().map(|s| vec![s]));
            m.set_length(Some(Time::from_millis(
                track.properties.duration.as_millis() as i64,
            )));

            m
        } else {
            Metadata::new()
        };

        Ok(metadata)
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.volume() as f64)
    }

    async fn set_volume(&self, volume: Volume) -> zbus::Result<()> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.set_volume(volume as f32))
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.can_go_next())
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.can_go_prev())
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.get_current_track().is_some())
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.get_current_track().is_some())
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.get_current_track().is_some())
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

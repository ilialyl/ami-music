use std::time::Duration;

use ami_core::queue::loop_mode;
use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Time, TrackId, Volume,
    zbus::{self, fdo},
};

use crate::{
    commands::{Command, PlaybackCommand, QueueCommand},
    services::mpris::Mpris,
};

impl PlayerInterface for Mpris {
    async fn next(&self) -> fdo::Result<()> {
        let _ = self.command_tx.send(Command::Queue(QueueCommand::Next));
        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        let _ = self.command_tx.send(Command::Queue(QueueCommand::Prev));
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::Pause));
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::TogglePlay));
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::Play));
        Ok(())
    }

    async fn seek(&self, offset: Time) -> fdo::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::Seek {
                offset_seconds: offset.as_secs() as i64,
            }));
        Ok(())
    }

    async fn set_position(&self, _track_id: TrackId, position: Time) -> fdo::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::SetPosition(
                Duration::from_micros(position.as_micros() as u64),
            )));
        Ok(())
    }
    async fn position(&self) -> fdo::Result<Time> {
        let orchestrator = self.shared_state.read().await;

        Ok(Time::from_micros(
            orchestrator.player_position().as_micros() as i64,
        ))
    }

    async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        let orchestrator = self.shared_state.read().await;
        let status = Mpris::match_playback_status(orchestrator.playback_status());
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
        let _ = self
            .command_tx
            .send(Command::Queue(QueueCommand::SetLoopMode(loop_mode)));

        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
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

        Ok(orchestrator.current_metadata())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.volume() as f64)
    }

    async fn set_volume(&self, volume: Volume) -> zbus::Result<()> {
        let _ = self
            .command_tx
            .send(Command::Playback(PlaybackCommand::SetVolume {
                value: volume as f32,
            }));
        Ok(())
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
        Ok(orchestrator.current_track().is_some())
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.current_track().is_some())
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        let orchestrator = self.shared_state.read().await;
        Ok(orchestrator.current_track().is_some())
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

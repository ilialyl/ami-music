use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Time, TrackId, Volume,
    zbus::{self, fdo},
};

use crate::services::mpris::Mpris;

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
        Ok(PlaybackStatus::Stopped)
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        Ok(LoopStatus::None)
    }

    async fn set_loop_status(&self, _loop_status: LoopStatus) -> zbus::Result<()> {
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
        Ok(Metadata::new())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        Ok(1.0)
    }

    async fn set_volume(&self, _volume: Volume) -> zbus::Result<()> {
        Ok(())
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(1.0)
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        Ok(true)
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(true)
    }
}

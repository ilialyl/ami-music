use std::{
    collections::HashMap,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use ami_core::{
    config::LibraryConfig,
    library::{Library, TrackId},
    player::{Playback, playback_snapshot::PlayerSnapshot, playback_status::PlaybackStatus},
    queue::{Queue, loop_mode::LoopMode},
    track::Track,
};
use anyhow::Result;
use mpris_server::{Metadata, Property, Time};
use rodio::{Player, source::EmptyCallback};
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use url::Url;

use crate::{
    app::MprisServer,
    events::ServerEvent,
    internal_events::InternalEvent,
    services::{
        COVER_ADDR,
        mpris::{BUS_NAME, Mpris},
    },
};

pub struct Orchestrator {
    playback: Arc<Playback>,
    queue: Queue,
    library: Library,
    internal_event_tx: UnboundedSender<InternalEvent>,
}

impl Orchestrator {
    pub fn new(tx: UnboundedSender<InternalEvent>) -> Result<Self> {
        println!("Loading Playback...");
        let playback = Arc::new(Playback::new()?);
        println!("Loading Queue...");
        let queue = Queue::default();
        println!("Loading Library...");
        let library = Library::default();
        Ok(Orchestrator {
            playback,
            queue,
            library,
            internal_event_tx: tx,
        })
    }

    fn load_track(&self, audio_path: &Path) -> Result<()> {
        let tx = self.internal_event_tx.clone();
        self.playback.load_track(audio_path)?;
        let fired = Arc::new(AtomicBool::new(false));
        self.playback
            .player
            .append(EmptyCallback::new(Box::new(move || {
                if fired
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    let _ = tx.send(InternalEvent::TrackEnded);
                }
            })));

        Ok(())
    }

    pub async fn play(&self, mpris_server: &Option<MprisServer>) -> Result<()> {
        if self.queue.current_track.is_some() && self.playback.player.empty() {
            self.rewind()?;
        }

        self.playback.play();
        if let Some(mpris_server) = mpris_server {
            mpris_server
                .read()
                .await
                .properties_changed([Property::PlaybackStatus(Mpris::match_playback_status(
                    self.playback_status(),
                ))])
                .await?
        }

        Ok(())
    }

    pub async fn pause(&self, mpris_server: &Option<MprisServer>) -> Result<()> {
        self.playback.pause();
        if let Some(mpris_server) = mpris_server {
            mpris_server
                .read()
                .await
                .properties_changed([Property::PlaybackStatus(Mpris::match_playback_status(
                    self.playback_status(),
                ))])
                .await?
        };

        Ok(())
    }

    pub async fn toggle_play(&self, mpris_server: &Option<MprisServer>) -> Result<()> {
        if self.queue.current_track.is_some() && self.playback.player.empty() {
            self.rewind()?;
            self.playback.play();
        } else {
            self.playback.toggle_play();
        }
        if let Some(mpris_server) = mpris_server {
            mpris_server
                .read()
                .await
                .properties_changed([Property::PlaybackStatus(Mpris::match_playback_status(
                    self.playback_status(),
                ))])
                .await?
        }

        Ok(())
    }

    pub fn set_position(&self, pos: Duration) -> Result<()> {
        self.playback.set_position(pos)?;
        Ok(())
    }

    pub fn seek(&self, offset_seconds: i64) -> Result<()> {
        self.playback.seek(offset_seconds)?;
        Ok(())
    }

    pub fn rewind(&self) -> Result<()> {
        if let Some(track) = self.queue.current_track.as_ref() {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
        }

        Ok(())
    }

    pub fn increase_volume(&self, step: f32) {
        self.playback.increase_volume(step);
    }

    pub fn decrease_volume(&self, step: f32) {
        self.playback.decrease_volume(step);
    }

    pub fn volume(&self) -> f32 {
        self.playback.volume()
    }

    pub fn set_volume(&self, value: f32) {
        self.playback.set_volume(value);
    }

    pub fn player_position(&self) -> Duration {
        self.playback.player.get_pos()
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        self.playback.playback_status()
    }

    pub fn playback_speed(&self) -> f32 {
        self.playback.playback_speed()
    }

    pub async fn send_player_position(
        player: Arc<Player>,
        connection_tx: &broadcast::Sender<String>,
    ) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            if !player.is_paused() {
                let event = ServerEvent::SendPlayerPosition(player.get_pos());
                let _ = connection_tx.send(serde_json::to_string(&event)?);
            }
        }
    }

    pub fn get_player_snapshot(&self) -> PlayerSnapshot {
        self.playback.get_snapshot()
    }

    pub async fn enqueue(&mut self, id: TrackId, mpris_server: &Option<MprisServer>) -> Result<()> {
        if let Some(track) = self.library.tracks.get(&id).cloned() {
            self.queue.enqueue(track.clone());

            log::debug!("Called Orchestrator::enqueue");
            if self.queue.current_track.is_none() {
                self.next(mpris_server).await?;
            } else if self.queue.current_track.is_some() && self.playback.player.empty() {
                self.next(mpris_server).await?;
            } else if self.playback.player.empty() {
                self.next(mpris_server).await?;
            }
        }

        Ok(())
    }

    pub fn prepend(&mut self, id: TrackId) {
        if let Some(track) = self.library.tracks.get(&id) {
            self.queue.prepend_queue(track.clone())
        }
    }

    pub fn dequeue(&mut self, index: usize) {
        self.queue.dequeue(index);
    }

    pub async fn play_now(
        &mut self,
        track_id: TrackId,
        mpris_server: &Option<MprisServer>,
    ) -> Result<()> {
        self.prepend(track_id);
        self.next(mpris_server).await?;

        Ok(())
    }

    pub fn can_go_next(&self) -> bool {
        !self.queue.next_tracks.is_empty()
    }

    pub fn can_go_prev(&self) -> bool {
        !self.queue.previous_tracks.is_empty()
    }

    pub async fn next(&mut self, mpris_server: &Option<MprisServer>) -> Result<bool> {
        if self.queue.next()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
            self.play(mpris_server).await?;
            if let Some(mpris_server) = mpris_server {
                mpris_server
                    .read()
                    .await
                    .properties_changed([
                        Property::Metadata(self.current_metadata()?),
                        Property::CanPlay(true),
                        Property::CanPause(true),
                        Property::CanSeek(true),
                    ])
                    .await?
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn prev(&mut self) -> Result<()> {
        if self.queue.prev()
            && let Some(track) = self.queue.current_track.as_ref()
        {
            self.playback.player.clear();
            self.load_track(&track.pathbuf)?;
            self.playback.play();
        }

        Ok(())
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle();
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) {
        self.queue.loop_mode = loop_mode;
    }

    pub fn loop_mode(&self) -> LoopMode {
        self.queue.loop_mode
    }

    pub fn cycle_loop_mode(&mut self) {
        self.queue.cycle_loop_mode();
    }

    pub fn current_track(&self) -> Option<Arc<Track>> {
        self.queue.current_track.clone()
    }

    pub fn current_metadata(&self) -> Result<Metadata> {
        if let Some(track) = self.current_track() {
            let mut m = Metadata::new();
            m.set_title(Some(track.metadata.title.clone()));
            m.set_album(track.metadata.album.clone());
            m.set_artist(track.metadata.artist.clone().map(|s| vec![s]));
            m.set_art_url(
                track
                    .metadata
                    .thumbnail_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                    .and_then(|name| Url::parse(&format!("http://{}/{}", COVER_ADDR, name)).ok()),
            );
            m.set_disc_number(track.metadata.disc_number.map(|n| n as i32));
            m.set_genre(track.metadata.genre.clone().map(|s| vec![s]));
            m.set_length(Some(Time::from_millis(
                track.properties.duration.as_millis() as i64,
            )));
            m.set_trackid(Some(mpris_server::TrackId::try_from(format!(
                "/org/mpris/MediaPlayer2/{}/track/{}",
                BUS_NAME,
                track.id.as_u64()
            ))?));

            Ok(m)
        } else {
            Ok(Metadata::new())
        }
    }

    pub fn restart_queue(&mut self) {
        self.queue.restart();
    }

    pub fn clone_queue(&self) -> Queue {
        self.queue.clone()
    }

    pub fn clone_library(&self) -> HashMap<TrackId, Arc<Track>> {
        self.library.tracks.clone()
    }

    pub fn clone_player_arc(&self) -> Arc<Player> {
        self.playback.player.clone()
    }

    pub fn load_library_config(&mut self, library_config: LibraryConfig) {
        self.library.load(library_config);
    }
}

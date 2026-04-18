use std::path::Path;

use crate::player::{Playback, playback_status::PlaybackStatus};

#[test]
fn new() {
    assert!(Playback::new().is_ok())
}

#[test]
fn stopped() {
    let player = Playback::new().unwrap();
    assert_eq!(player.playback_status(), PlaybackStatus::Stopped);
}

#[tokio::test]
async fn play() {
    let player = Playback::new().unwrap();
    player
        .load_track(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.play();
    assert_eq!(player.playback_status(), PlaybackStatus::Playing);
}

#[tokio::test]
async fn pause() {
    let player = Playback::new().unwrap();
    player
        .load_track(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.pause();
    assert_eq!(player.playback_status(), PlaybackStatus::Paused);
}

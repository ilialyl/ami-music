use std::path::Path;

use mpris_server::PlaybackStatus;

use crate::player::Playback;

#[test]
fn new() {
    assert!(Playback::new().is_ok())
}

#[test]
fn stopped() {
    let player = Playback::new().unwrap();
    assert_eq!(player.playback_status(), PlaybackStatus::Stopped);
}

#[test]
fn play() {
    let player = Playback::new().unwrap();
    player
        .load_track(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.play();
    assert_eq!(player.playback_status(), PlaybackStatus::Playing);
}

#[test]
fn pause() {
    let player = Playback::new().unwrap();
    player
        .load_track(Path::new("../test_assets/test.flac"))
        .unwrap();
    player.pause();
    assert_eq!(player.playback_status(), PlaybackStatus::Paused);
}

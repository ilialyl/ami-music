use std::path::PathBuf;

use crate::{queue::Queue, track::Track};

#[test]
fn next() {
    let mut queue = Queue::default();
    let pathbuf_1 = PathBuf::from("../test_assets/test.flac");
    let pathbuf_2 = PathBuf::from("../test_assets/test1.flac");

    queue.enqueue(Track::new(pathbuf_1.as_path(), 1).unwrap());
    queue.enqueue(Track::new(pathbuf_2.as_path(), 2).unwrap());

    assert!(queue.next());
    assert_eq!(queue.current_track.as_ref().unwrap().pathbuf, pathbuf_1);

    assert!(queue.next());
    assert!(queue.current_track.is_some());
    assert_eq!(queue.current_track.as_ref().unwrap().pathbuf, pathbuf_2);

    assert!(!queue.next());
    assert_eq!(queue.current_track.as_ref().unwrap().pathbuf, pathbuf_2);
}

#[test]
fn prev() {
    let mut queue = Queue::default();
    let pathbuf_1 = PathBuf::from("../test_assets/test.flac");
    let pathbuf_2 = PathBuf::from("../test_assets/test1.flac");

    queue.enqueue(Track::new(pathbuf_1.as_path(), 1).unwrap());
    queue.enqueue(Track::new(pathbuf_2.as_path(), 2).unwrap());

    assert!(!queue.prev());
    assert!(queue.current_track.is_none());

    queue.next();
    queue.next();

    assert!(queue.prev());
    assert_eq!(queue.current_track.unwrap().pathbuf, pathbuf_1);
}

#[test]
fn prepend() {
    let mut queue = Queue::default();
    let pathbuf_1 = PathBuf::from("../test_assets/test.flac");
    let pathbuf_2 = PathBuf::from("../test_assets/test1.flac");

    queue.enqueue(Track::new(pathbuf_1.as_path(), 1).unwrap());
    queue.prepend_queue(Track::new(pathbuf_2.as_path(), 2).unwrap());
    queue.next();
    assert_eq!(queue.current_track.unwrap().pathbuf, pathbuf_2);
}

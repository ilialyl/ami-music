use snowy_core::{library::Library, queue::Queue};

#[derive(Default)]
pub struct AppState {
    pub queue: Queue,
    pub library: Library,
}

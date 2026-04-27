use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    widgets::{ListState, StatefulWidget, TableState, Widget},
};

use crate::{
    app::App,
    ui::{cover_art::CoverArt, library::Library, now_playing::NowPlaying, queue::Queue},
};

pub mod cover_art;
pub mod library;
pub mod now_playing;
pub mod queue;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Fill(1),
                Constraint::Percentage(20),
            ])
            .margin(1)
            .spacing(1)
            .split(area);

        let playing_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(layout[0].width / 2),
                Constraint::Min(0),
            ])
            .split(layout[0]);

        if let Ok(daemon_states) = self.daemon_states.try_lock().as_mut() {
            let cover_art = CoverArt {};

            if let Some(protocol) = daemon_states.cover_art.as_mut() {
                cover_art.render(playing_panel[0], buf, protocol);
            }

            let playing_desc = NowPlaying {
                daemon_states: &daemon_states,
            };
            playing_desc.render(playing_panel[1].inner(Margin::new(1, 0)), buf);

            let queue = Queue {
                daemon_states: &daemon_states,
            };
            queue.render(layout[2], buf, &mut ListState::default());

            let library = Library {
                daemon_states: &daemon_states,
            };
            library.render(
                layout[1].inner(Margin::new(1, 0)),
                buf,
                &mut TableState::default(),
            );
        }
    }
}

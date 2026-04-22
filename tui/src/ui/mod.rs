use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{ListState, StatefulWidget, TableState, Widget},
};

use crate::{
    app::App,
    ui::{library::Library, now_playing::NowPlaying, queue::Queue},
};

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
                Constraint::Percentage(30),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ])
            .split(area);

        let playing_panel = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(layout[0].width / 2),
                Constraint::Min(0),
            ])
            .split(layout[0]);

        let playing_desc = NowPlaying { app: &self };
        playing_desc.render(playing_panel[1], buf);

        let queue = Queue { app: &self };
        queue.render(layout[2], buf, &mut ListState::default());

        let library = Library { app: &self };
        library.render(layout[1], buf, &mut TableState::default());
    }
}

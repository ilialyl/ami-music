use std::ops::Mul;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Paragraph, Widget},
};

use crate::state::DaemonStates;

pub struct NowPlaying<'a> {
    pub daemon_states: &'a DaemonStates,
}

impl<'a> Widget for NowPlaying<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if let Some(track) = self.daemon_states.queue_snapshot.current_track.as_ref() {
            let position = self.daemon_states.player_snapshot.position.clone();
            let desc_lines = [
                Some(track.metadata.title.clone()),
                Some(
                    track
                        .metadata
                        .artist
                        .clone()
                        .unwrap_or("Unknown Artist".into()),
                ),
                match (track.metadata.album.clone(), track.metadata.year) {
                    (Some(album), Some(year)) => Some(format!("{} ({})", album, year)),
                    (Some(album), None) => Some(album.into()),
                    (None, Some(year)) => Some(format!("{}", year)),
                    (None, None) => None,
                },
                Some(format!(
                    "{}:{:02}",
                    position.as_secs().saturating_div(60),
                    position.as_secs() % 60
                )),
            ];

            let flat_desc = desc_lines.into_iter().flatten().collect::<Vec<String>>();
            let spacing = (1..=5)
                .rev()
                .find(|&x| (area.height / 2) >= (flat_desc.len().mul(x).saturating_sub(1) as u16))
                .unwrap_or(1);

            let sep = "\n".repeat(spacing);
            let desc = flat_desc.join(&sep);

            let paragraph = Paragraph::new(desc);

            paragraph.render(
                area.centered_vertically(Constraint::Length(area.height / 2)),
                buf,
            );
        }
    }
}

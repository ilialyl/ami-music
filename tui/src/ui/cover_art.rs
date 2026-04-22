use std::{io::Cursor, sync::Arc};

use color_eyre::eyre::Result;
use image::{DynamicImage, GenericImageView, ImageReader};
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use ratatui_image::{Resize, StatefulImage, picker::Picker, protocol::StatefulProtocol};
use url::Url;

use crate::app::App;

pub struct CoverArt<'a> {
    pub app: &'a App,
}

impl<'a> CoverArt<'a> {
    pub async fn parse_cover_art(
        url: Url,
        picker: Arc<Picker>,
    ) -> Result<Option<StatefulProtocol>> {
        let bytes = reqwest::get(url).await?.bytes().await?;
        if let Some(dyn_img) = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.decode().ok())
        {
            log::debug!("Created Dynamic Image.");
            let protocol = picker.new_resize_protocol(Self::crop_to_square(dyn_img));
            log::debug!("Cropped to square.");
            Ok(Some(protocol))
        } else {
            Ok(None)
        }
    }

    fn crop_to_square(dyn_img: DynamicImage) -> DynamicImage {
        let (width, height) = dyn_img.dimensions();

        if width.eq(&height) {
            return dyn_img;
        }

        let size = width.min(height);
        let x = (width - size) / 2;
        let y = (height - size) / 2;

        dyn_img.crop_imm(x, y, size, size)
    }
}

impl<'a> StatefulWidget for CoverArt<'a> {
    type State = StatefulProtocol;
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        if let Ok(states) = self.app.states.try_lock().as_mut() {
            if let Some(protocol) = states.cover_art.as_mut() {
                let widget = StatefulImage::default().resize(Resize::Scale(None));
                StatefulWidget::render(widget, area, buf, protocol);
            }
        }
    }
}

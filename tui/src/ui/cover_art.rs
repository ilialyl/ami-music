use std::{io::Cursor, sync::Arc};

use color_eyre::eyre::Result;
use image::{DynamicImage, GenericImageView, ImageReader};
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use ratatui_image::{Resize, StatefulImage, picker::Picker, protocol::StatefulProtocol};
use url::Url;

pub struct CoverArt {}

impl CoverArt {
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

impl StatefulWidget for CoverArt {
    type State = StatefulProtocol;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let widget = StatefulImage::default().resize(Resize::Scale(None));
        StatefulWidget::render(widget, area, buf, state);
    }
}

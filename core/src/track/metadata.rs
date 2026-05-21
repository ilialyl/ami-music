use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use anyhow::Result;
use image::{ImageFormat, ImageReader, imageops::FilterType};
use lofty::{file::TaggedFileExt, picture::PictureType};
use serde::{Deserialize, Serialize};

use crate::cache::get_cover_art_cache_path;

const THUMB_SIZE: u32 = 1000;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub length: u128,
    pub album: Option<String>,
    pub title: String,
    pub artist: Option<String>,
    pub disc_number: Option<u32>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub cover_art_path: Option<PathBuf>,
}

impl Metadata {
    pub fn cover_art_path(audio_path: &Path) -> Result<PathBuf> {
        Ok(get_cover_art_cache_path()?.join(format!(
            "{}.jpg",
            audio_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
        )))
    }

    /// Cache the audio file's cover art and return PathBuf.
    pub fn cache_cover_art(audio_path: &Path) -> Option<PathBuf> {
        let thumb_path = Self::cover_art_path(audio_path).ok()?;
        if thumb_path.exists() {
            return Some(thumb_path);
        }

        log::debug!("Caching cover art for {:?}", audio_path);
        let tagged = lofty::read_from_path(audio_path).ok()?;
        let tag = tagged.primary_tag().or_else(|| tagged.first_tag())?;

        let picture = tag
            .pictures()
            .iter()
            .find(|p| p.pic_type() == PictureType::CoverFront)
            .or_else(|| tag.pictures().first())?;

        let img = ImageReader::new(Cursor::new(picture.data()))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?;

        let thumb = img.resize(THUMB_SIZE, THUMB_SIZE, FilterType::Lanczos3);

        thumb
            .save_with_format(&thumb_path, ImageFormat::Jpeg)
            .ok()?;

        Some(thumb_path)
    }
}

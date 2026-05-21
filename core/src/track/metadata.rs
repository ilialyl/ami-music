use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use anyhow::Result;
use image::{ImageFormat, ImageReader, imageops::FilterType};
use lofty::{file::TaggedFileExt, picture::PictureType};
use serde::{Deserialize, Serialize};

use crate::cache::get_cover_art_cache_path;

const COVER_ART_SIZE: u32 = 1000;

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
        let cover_art_path = Self::cover_art_path(audio_path).ok()?;
        if cover_art_path.exists() {
            return Some(cover_art_path);
        }

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

        log::debug!("Caching cover art for {:?}", audio_path);

        let cover_art = img.resize(COVER_ART_SIZE, COVER_ART_SIZE, FilterType::Lanczos3);

        cover_art
            .save_with_format(&cover_art_path, ImageFormat::Jpeg)
            .ok()?;

        Some(cover_art_path)
    }
}

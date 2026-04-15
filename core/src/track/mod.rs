pub mod metadata;
pub mod properties;

use std::path::{Path, PathBuf};

use anyhow::Result;
use lofty::{
    config::ParseOptions,
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use serde::{Deserialize, Serialize};

use crate::track::{metadata::Metadata, properties::Properties};

/// Stores necessary information about a track.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Track {
    pub pathbuf: PathBuf,
    pub metadata: Metadata,
    pub properties: Properties,
}

impl Track {
    pub fn new(path: &Path) -> Result<Self> {
        let tagged_file = Probe::open(path)?
            .options(ParseOptions::new().read_cover_art(false))
            .read()?;
        Ok(Self {
            pathbuf: path.to_path_buf(),
            metadata: Self::parse_metadata(&tagged_file)?,
            properties: Self::parse_properties(&tagged_file),
        })
    }

    /// Parses metadata from TaggedFile to Metadata type that comes with mpris-server crate.
    fn parse_metadata(tagged_file: &TaggedFile) -> Result<Metadata> {
        let mut metadata = Metadata::default();
        metadata.length = tagged_file.properties().duration().as_micros();
        if let Some(primary_tag) = tagged_file.primary_tag() {
            metadata.album = primary_tag.album().map(|s| s.into_owned());
            metadata.title = primary_tag.title().map(|s| s.into_owned());
            metadata.artist = primary_tag.artist().map(|s| s.into_owned());
            metadata.disc_number = primary_tag.track();
            metadata.genre = primary_tag.genre().map(|s| s.into_owned());
        }

        Ok(metadata)
    }

    /// Stores TaggedFile's Properties in a custom Properties type.
    fn parse_properties(tagged_file: &TaggedFile) -> Properties {
        Properties {
            bitrate: tagged_file.properties().audio_bitrate(),
            sample_rate: tagged_file.properties().sample_rate(),
            bit_depth: tagged_file.properties().bit_depth(),
            channels: tagged_file.properties().channels(),
        }
    }
}

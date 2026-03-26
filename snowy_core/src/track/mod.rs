pub mod properties;

use std::path::{Path, PathBuf};

use anyhow::Result;
use lofty::{
    config::ParseOptions,
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};
use mpris_server::{Metadata, Time};

use crate::track::properties::Properties;

/// Stores necessary information about a track.
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
        let mut metadata = Metadata::new();
        metadata.set_length(Some(Time::from_micros(
            tagged_file.properties().duration().as_micros() as i64,
        )));
        if let Some(primary_tag) = tagged_file.primary_tag() {
            metadata.set_album(primary_tag.album().as_deref());
            metadata.set_title(primary_tag.title().as_deref());
            metadata.set_artist(primary_tag.artist().as_deref().map(|s| vec![s]));
            metadata.set_disc_number(primary_tag.track().map(|i| i as i32));
            metadata.set_genre(primary_tag.genre().as_deref().map(|s| vec![s]));
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

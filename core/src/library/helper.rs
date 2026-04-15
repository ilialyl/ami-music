use std::path::Path;

use anyhow::{Result, anyhow};

pub const RODIO_SUPPORTED_FORMATS: [&str; 6] = ["flac", "mp3", "ogg", "wav", "opus", "m4a"];

pub fn is_rodio_supported(path: &Path) -> Result<bool> {
    if path.is_file() {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if RODIO_SUPPORTED_FORMATS.contains(&extension) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(anyhow!("file has no extension"))
        }
    } else {
        Err(anyhow!("path is not a file"))
    }
}

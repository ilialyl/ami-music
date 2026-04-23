use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use crate::APP_NAME;

#[derive(Deserialize)]
pub struct Config {
    pub library: LibraryConfig,
}

#[derive(Deserialize)]
pub struct LibraryConfig {
    pub directories: Vec<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = dirs::config_dir()
            .unwrap()
            .join(format!("{}.toml", APP_NAME));
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}

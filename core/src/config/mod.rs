use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub library: LibraryConfig,
}

#[derive(Deserialize)]
pub struct LibraryConfig {
    pub directories: Vec<PathBuf>,
}

impl Config {
    #[cfg(debug_assertions)]
    pub fn load() -> Result<Self> {
        let path = PathBuf::from("/home/lyns0/projects/personal/ami/ami_config.toml");
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
    #[cfg(not(debug_assertions))]
    pub fn load() -> Result<Self> {
        let path = dirs::config_dir().unwrap().join("snowy_config.toml");
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}

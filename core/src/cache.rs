use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;

use crate::APP_NAME;

pub fn get_cache_path() -> Result<PathBuf> {
    let path = dirs::cache_dir().unwrap().join(APP_NAME);
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn get_thumbnail_cache_path() -> Result<PathBuf> {
    let path = get_cache_path()?.join("thumbnails");
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

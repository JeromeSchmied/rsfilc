use crate::config::APP_NAME;
use std::{fs, path::PathBuf};

/// get path for cache dir, create if doesn't exist
/// # Errors
/// `cache_dir` creation
pub fn cache_dir() -> Option<PathBuf> {
    let cache_path = dirs::cache_dir()?.join(APP_NAME);
    if !cache_path.exists() {
        fs::create_dir_all(&cache_path).ok()?;
    }
    Some(cache_path)
}

/// get cache path for `kind` of thing
pub fn cache_path(kind: &str) -> Option<PathBuf> {
    Some(cache_dir()?.join(format!("{kind}_cache.json")))
}

/// get path for `Downloads/rsfilc`, and create it if doesn't exist yet
///
/// # Panics
///
/// no `Downloads`
pub fn download_dir() -> PathBuf {
    let dl_dir = dirs::download_dir()
        .unwrap_or(dirs::home_dir().expect("no home dir").join("Downloads"))
        .join(APP_NAME);
    if !dl_dir.exists() {
        fs::create_dir_all(&dl_dir).unwrap();
    }
    dl_dir
}

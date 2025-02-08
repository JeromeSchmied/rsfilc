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
    let cache_path = cache_dir()?.join(format!("{kind}_cache.json"));
    cache_path.exists().then_some(cache_path)
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

#[cfg(test)]
mod tests {
    #[test]
    fn cache_path_exists() {
        assert!(super::cache_dir().is_some());
    }
    #[test]
    /// just check whether it panics
    fn dl_path_exists() {
        super::download_dir();
    }
}

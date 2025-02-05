use super::Res;
use log::{info, warn};
use std::fs::{self, File};
use std::path::PathBuf;

/// get path for saved user credentials
pub fn cred_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("rsfilc").join("credentials.toml"))
}

/// get path for config
pub fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("rsfilc").join("config.toml"))
}

/// get path for cache dir
///
/// # Panics
///
/// `cache_dir` creation
pub fn cache_dir() -> Option<PathBuf> {
    let cache_path = dirs::cache_dir()?.join("rsfilc");
    if !cache_path.exists() {
        fs::create_dir_all(cache_path).expect("couldn't create cache dir");
    }
    Some(dirs::cache_dir()?.join("rsfilc"))
}

pub fn cache_path(kind: &str) -> PathBuf {
    cache_dir().unwrap().join(format!("{}_cache.json", kind))
}

/// delete all cache and logs as well
pub fn delete_cache_dir() -> Res<()> {
    if let Some(cd) = cache_dir() {
        if cd.exists() {
            warn!("deleting cache dir");
            fs::remove_dir_all(cd)?;
            info!("done");
        }
    }
    Ok(())
}

/// get log file with the help of [`log_path()`]
pub fn log_file(kind: &str) -> Res<File> {
    Ok(File::create(log_path(kind))?)
}

/// get log path for `kind`: `kind`.log
///
/// # Panics
///
/// no `cache_path`
pub fn log_path(kind: &str) -> PathBuf {
    cache_dir()
        .expect("couldn't find cache path")
        .join([kind, ".log"].concat())
}

/// get path for `Downloads/rsfilc`, and create it if doesn't exist yet
///
/// # Panics
///
/// no `Downloads`
pub fn download_dir() -> PathBuf {
    let dl_dir = if let Some(default_dl) = dirs::download_dir() {
        default_dl.join("rsfilc")
    } else if let Some(home) = dirs::home_dir() {
        home.join("Downloads").join("rsfilc")
    } else {
        panic!("couldn't find Downloads directory");
    };
    if !dl_dir.exists() {
        fs::create_dir_all(&dl_dir).unwrap();
    }
    dl_dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_path_exists() {
        assert!(cache_dir().is_some());
    }
    #[test]
    fn config_path_exists() {
        assert!(config_path().is_some());
    }
    #[test]
    fn cred_path_exists() {
        assert!(cred_path().is_some());
    }
    #[test]
    /// just check whether it panics
    fn dl_path_exists() {
        download_dir();
    }
}

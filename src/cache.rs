use crate::{paths::cache_path, Res};
use chrono::{DateTime, Local};
use ekreta::LDateTime;
use std::fs::{self, File};
use std::io::Write;

/// save to disk
pub fn store(kind: &str, content: &str) -> Res<()> {
    let cp = cache_path(kind).ok_or("couldn't get cache path")?;
    // let mut f = OpenOptions::new().create(true).append(true).open(&cp)?;
    let mut f = File::create(&cp)?;
    log::info!("caching to {cp:?}");

    // let content = serde_json::to_string(content)?;
    writeln!(f, "{}", Local::now().to_rfc3339())?;
    // f.write_all(content.as_bytes())?;
    writeln!(f, "{content}")?;

    Ok(())
}

/// load from disk
pub fn load(kind: &str) -> Option<(LDateTime, String)> {
    let cp = cache_path(kind)?;
    if !cp.exists() {
        return None;
    }
    log::info!("loading cache from {cp:?}");
    let content = fs::read_to_string(cp).unwrap_or_default();
    let mut cl = content.lines().collect::<Vec<&str>>();
    let t = cl.remove(0);
    let t = DateTime::parse_from_rfc3339(t).ok()?;

    let c = cl.iter().fold(String::new(), |all, cur| all + cur);
    // let x = serde_json::from_str(&c)?;

    Some((t.into(), c))
}
/// delete all cache and logs as well
pub fn delete_dir() -> Res<()> {
    if let Some(cd) = crate::paths::cache_dir() {
        if cd.exists() {
            log::warn!("deleting cache dir");
            fs::remove_dir_all(cd)?;
            log::info!("done");
        }
    }
    Ok(())
}

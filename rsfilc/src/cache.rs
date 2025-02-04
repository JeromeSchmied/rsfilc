use crate::{paths::cache_path, Res};
use chrono::{DateTime, Local};
use ekreta::LDateTime;
use log::info;
use std::fs::{self, File};
use std::io::Write;

/// save to disk
pub fn store(kind: &str, content: &str) -> Res<()> {
    let cp = cache_path(kind);
    // let mut f = OpenOptions::new().create(true).append(true).open(&cp)?;
    let mut f = File::create(&cp)?;
    info!("caching to {cp:?}");

    // let content = serde_json::to_string(content)?;
    writeln!(f, "{}", Local::now().to_rfc3339())?;
    // f.write_all(content.as_bytes())?;
    writeln!(f, "{}", content)?;

    Ok(())
}

/// load from disk
pub fn load(kind: &str) -> Option<(LDateTime, String)> {
    let cp = cache_path(kind);
    if !cp.exists() {
        return None;
    }
    info!("loading cache from {cp:?}");
    let content = if let Ok(cont) = fs::read_to_string(cp) {
        cont
    } else {
        String::new()
    };
    let mut cl = content.lines().collect::<Vec<&str>>();
    let t = cl.remove(0);
    let t = DateTime::parse_from_rfc3339(t).ok()?;

    let c = cl.iter().fold(String::new(), |all, cur| all + cur);
    // let x = serde_json::from_str(&c)?;

    Some((t.into(), c))
}

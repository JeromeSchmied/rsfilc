//! `RsFilc`: `Kréta` API and client

use chrono::{DateTime, Datelike, Local, Timelike};
use log::*;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub mod absences;
pub mod announced;
mod api_urls;
pub mod args;
pub mod endpoints;
pub mod evals;
pub mod information;
pub mod messages;
pub mod school_list;
pub mod timetable;
pub mod token;
pub mod user;

// reexports
pub use absences::Abs;
pub use announced::Ancd;
pub use args::{Args, Commands};
pub use evals::Eval;
pub use school_list::School;
pub use timetable::Lesson;
pub use user::User;

/// Result from `T` and `Box<dyn Error>`
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

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
pub fn cache(kind: &str, content: &str) -> Res<()> {
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
pub fn uncache(kind: &str) -> Option<(DateTime<Local>, String)> {
    let cp = cache_path(kind);
    if !cp.exists() {
        return None;
    }
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

/// `DateTime` methods needed for `RsFilc`
pub trait MyDate {
    /// Practical date format.
    fn pretty(&self) -> String;
    /// Get hungarian month.
    fn hun_month<'a>(&self) -> &'a str;
    /// Get hungarian day of week.
    fn hun_day_of_week<'a>(&self) -> &'a str;
    fn make_kreta_valid(&self) -> String;
}
impl MyDate for DateTime<Local> {
    fn pretty(&self) -> String {
        let this_year = self.year() == Local::now().year();
        let day_diff = self.num_days_from_ce() - Local::now().num_days_from_ce();

        if !this_year {
            format!("{}", self.format("%Y.%m.%d"))
        } else if day_diff == -1 {
            format!(
                "tegnap{}",
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    ""
                } else {
                    " %H:%M"
                })
            )
        } else if day_diff == 1 {
            format!(
                "holnap{}",
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    ""
                } else {
                    " %H:%M"
                })
            )
        } else if day_diff == 0 {
            format!(
                "ma{}",
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    ""
                } else {
                    " %H:%M"
                })
            )
        } else {
            format!(
                "{} {}",
                self.hun_month(),
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    "%d."
                } else {
                    "%d. %H:%M"
                })
            )
        }
    }

    fn hun_month<'a>(&self) -> &'a str {
        match self.month() {
            1 => "jan.",
            2 => "feb.",
            3 => "már.",
            4 => "ápr.",
            5 => "máj.",
            6 => "jún.",
            7 => "júl.",
            8 => "aug.",
            9 => "szep.",
            10 => "okt.",
            11 => "nov.",
            12 => "dec.",
            _ => unreachable!("invalid month"),
        }
    }

    fn hun_day_of_week<'a>(&self) -> &'a str {
        match self.weekday().number_from_monday() {
            1 => "hétfő",
            2 => "kedd",
            3 => "szerda",
            4 => "csütörtök",
            5 => "péntek",
            6 => "szombat",
            7 => "vasárnap",
            _ => unreachable!("invalid day of week"),
        }
    }

    fn make_kreta_valid(&self) -> String {
        self.date_naive().and_hms_opt(0, 0, 0).unwrap().to_string()
    }
}

/// Fill under `fill` with many `with` [`char`]s.
pub fn fill_under(fill: &str, with: char) {
    let longest = fill.lines().max_by_key(|l| l.chars().count()).unwrap_or("");
    println!("\n{}", with.to_string().repeat(longest.chars().count()));
}

#[cfg(test)]
mod tests;

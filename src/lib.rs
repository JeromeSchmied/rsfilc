//! RsFilc: `Kréta` API and client

use chrono::{DateTime, Datelike, Local, Timelike};
use log::*;
use std::{
    fs::{self, File},
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
pub fn cache_path() -> Option<PathBuf> {
    let cache_path = dirs::cache_dir()?.join("rsfilc");
    if !cache_path.exists() {
        fs::create_dir_all(cache_path).expect("couldn't create cache dir");
    }
    Some(dirs::cache_dir()?.join("rsfilc"))
}
/// get log file with the help of [`log_path()`]
pub fn log_file(kind: &str) -> Res<File> {
    Ok(File::create(log_path(kind))?)
}
/// get log path for `kind`: `kind`.log
pub fn log_path(kind: &str) -> PathBuf {
    cache_path()
        .expect("couldn't find cache path")
        .join([kind, ".log"].concat())
}
/// get path for `Downloads/rsfilc`, and create it if doesn't exist yet
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

/// DateTime methods needed for RsFilc
pub trait MyDate {
    /// Practical date format.
    fn pretty(&self) -> String;
    /// Get hungarian month.
    fn hun_month<'a>(&self) -> &'a str;
    /// Get hungarian day of week.
    fn hun_day_of_week<'a>(&self) -> &'a str;
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
}

/// Fill under `fill` with many `with` [`char`]s.
pub fn fill_under(fill: &str, with: char) {
    let longest = fill.lines().max_by_key(|l| l.chars().count()).unwrap_or("");
    println!("\n{}", with.to_string().repeat(longest.chars().count()));
}

#[cfg(test)]
mod tests;

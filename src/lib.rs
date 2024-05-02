//! RsFilc: `Kréta` API and client

use chrono::{DateTime, Datelike, Local, Timelike};
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
pub mod info;
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

/// format date so it looks pretty with hungarian text
pub fn pretty_date(date: &DateTime<Local>) -> String {
    let this_year = date.year() == Local::now().year();
    let day_diff = date.num_days_from_ce() - Local::now().num_days_from_ce();

    if !this_year {
        format!("{}", date.format("%Y.%m.%d"))
    } else if day_diff == -1 {
        format!(
            "tegnap{}",
            date.format(if date.hour() == 0 && date.minute() == 0 {
                ""
            } else {
                " %H:%M"
            })
        )
    } else if day_diff == 1 {
        format!(
            "holnap{}",
            date.format(if date.hour() == 0 && date.minute() == 0 {
                ""
            } else {
                " %H:%M"
            })
        )
    } else if day_diff == 0 {
        format!(
            "ma{}",
            date.format(if date.hour() == 0 && date.minute() == 0 {
                ""
            } else {
                " %H:%M"
            })
        )
    } else {
        format!(
            "{} {}",
            month(date.month().try_into().unwrap()),
            date.format(if date.hour() == 0 && date.minute() == 0 {
                "%d."
            } else {
                "%d. %H:%M"
            })
        )
    }
}
/// converts from month as number to month as hungarian text
pub fn month<'a>(m: u8) -> &'a str {
    match m {
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
/// converts from day as number of week to day as hungarian text
pub fn day_of_week<'a>(d: u8) -> &'a str {
    match d {
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

/// Fill under `fill` with many `with` [`char`]s.
pub fn fill_under(fill: &str, with: char) {
    let longest = fill.lines().max_by_key(|l| l.chars().count()).unwrap_or("");
    println!("\n{}", with.to_string().repeat(longest.chars().count() + 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_path_exists() {
        assert!(cache_path().is_some());
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

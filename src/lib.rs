//! RsFilc: `Kréta` API and client

use chrono::{DateTime, Datelike, Local, Timelike};

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

/// Result from `T` and `Box<dyn Error>`
pub type AnyErr<T> = Result<T, Box<dyn std::error::Error>>;

/// format date so it looks pretty with hungarian text
pub fn pretty_date(date: &DateTime<Local>) -> String {
    let this_year = date.year() == Local::now().year();

    if !this_year {
        format!("{}", date.format("%Y/%m/%d"))
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
pub fn month(m: u8) -> String {
    match m {
        1 => "jan.".to_string(),
        2 => "feb.".to_string(),
        3 => "már.".to_string(),
        4 => "ápr.".to_string(),
        5 => "máj.".to_string(),
        6 => "jún.".to_string(),
        7 => "júl.".to_string(),
        8 => "aug.".to_string(),
        9 => "szep.".to_string(),
        10 => "okt.".to_string(),
        11 => "nov.".to_string(),
        12 => "dec.".to_string(),
        _ => unreachable!("invalid month"),
    }
}
/// converts from day as number of week to day as hungarian text
pub fn day_of_week(d: u8) -> String {
    match d {
        1 => "hétfő".to_string(),
        2 => "kedd".to_string(),
        3 => "szerda".to_string(),
        4 => "csütörtök".to_string(),
        5 => "péntek".to_string(),
        6 => "szombat".to_string(),
        7 => "vasárnap".to_string(),
        _ => unreachable!("invalid day of week"),
    }
}

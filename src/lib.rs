use chrono::{DateTime, Datelike, Local};

pub mod absences;
pub mod announced;
pub mod api;
mod api_urls;
pub mod args;
pub mod evals;
pub mod info;
pub mod messages;
pub mod school_list;
pub mod timetable;
pub mod token;

/// Result from `T` and `Box<dyn Error>`
pub type AnyErr<T> = Result<T, Box<dyn std::error::Error>>;

pub fn pretty_date(date: &DateTime<Local>) -> String {
    let this_year = date.year() == Local::now().year();

    if !this_year {
        format!("{}", date.format("%Y/%m/%d"))
    } else {
        format!(
            "{} {}",
            month(date.month().try_into().unwrap()),
            date.format("%d. %H:%M")
        )
    }
}
fn month(m: u8) -> String {
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

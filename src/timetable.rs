//! lessons the student has

use crate::user::Usr;
use chrono::{Datelike, Duration, Local, NaiveDate};
use ekreta::{Lesson, Res};
use log::*;
use std::{fmt::Write, fs::File, io::Write as _, path::PathBuf};

pub fn handle(day: Option<&String>, user: &Usr, current: bool, out_p: Option<PathBuf>) -> Res<()> {
    let day = parse_day(day);
    let all_lessons_till_day = user.get_timetable(day, true)?;
    let lessons = user.get_timetable(day, false)?;
    if lessons.is_empty() {
        println!("{} ({}) nincs rögzített órád, juhé!", day, day.weekday());
        return Ok(());
    }
    if current {
        let current_lessons = current_lessons(&lessons);
        if let Some(nxt) = next_lesson(&all_lessons_till_day) {
            println!(
                "{}m -> {}",
                (nxt.kezdet_idopont - Local::now()).num_minutes(), // minutes remaining
                nxt.tantargy.nev
            );
        }
        for current_lesson in current_lessons {
            println!(
                "{}, {}m",
                current_lesson.tantargy.nev,
                (current_lesson.veg_idopont - Local::now()).num_minutes() // minutes remaining
            );
        }

        return Ok(());
    }
    if let Some(export_json_to) = out_p {
        info!("exported timetable to json");
        let mut f = File::create(export_json_to)?;
        let content = serde_json::to_string(&lessons)?;
        write!(f, "{content}")?;
    }
    user.print_day(lessons);
    Ok(())
}
/// Parse the day got as `argument`.
///
/// # Panics
///
/// Panics if
/// - day shifter contains invalid number.
/// - any datetime is invalid.
pub fn parse_day(day: Option<&String>) -> NaiveDate {
    info!("parsing day");
    if let Some(date) = day {
        let date = date.replace(['/', '.'], "-");
        info!("date: {date}");
        if let Ok(ndate) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            info!("valid as: {ndate}");
            ndate
        } else if date.starts_with('+') || date.ends_with('-') {
            info!("day_shift!");
            let day_shift = if date.starts_with('+') {
                info!("day_shift: +");
                date.parse::<i16>().expect("invalid +day shifter")
            } else {
                let date = &date[0..date.len() - 1];
                info!("day_shift: -");
                -date.parse::<i16>().expect("invalid day- shifter")
            };
            Local::now()
                .checked_add_signed(Duration::days(day_shift.into()))
                .expect("invalid datetime")
                .date_naive()
        } else if let Some(diff) = day_diff(&date) {
            /* eg.:
            today == thursday == 4
            looking_for == tuesday == 2
            (7 - today) + looking_for
            */
            Local::now()
                .checked_add_signed(Duration::days(diff.into()))
                .expect("invalid datetime")
                .date_naive()
        } else {
            info!("fallback: today");
            Local::now().date_naive()
        }
    } else {
        info!("fallback: today");
        Local::now().date_naive()
    }
}

/// Returns the current [`Lesson`]s of this [`User`] from `lessons` which shall include today's [`Lesson`]s.
///
/// # Warning
///
/// returns a `Vec<&Lesson>`, as a person might accidentally have more than one lessons at a time
pub fn current_lessons(lessons: &[Lesson]) -> Vec<&Lesson> {
    lessons.iter().filter(|lsn| lsn.happening()).collect()
}
/// Returns the next [`Lesson`] of this [`User`] from `lessons` which shall include today's [`Lesson`]s.
///
/// # Warning
///
/// There might accidentally be more next [`Lesson`]s. In this case only one of them is returned.
/// Also, if there is any `current_lesson`, None is returned
pub fn next_lesson(lessons: &[Lesson]) -> Option<&Lesson> {
    if !current_lessons(lessons).is_empty() {
        return None;
    }
    lessons
        .iter()
        .filter(|lsn| lsn.forecoming() && !lsn.kamu_smafu() && !lsn.cancelled())
        .collect::<Vec<_>>()
        .first()
        .copied()
}

pub fn disp(lsn: &Lesson) -> String {
    let mut f = String::new();
    _ = write!(&mut f, "{}", lsn.tantargy.nev);
    if let Some(room) = &lsn.terem_neve {
        _ = writeln!(&mut f, ", {}", room.replace("terem", "").trim());
    }

    if !lsn.kamu_smafu() {
        _ = writeln!(
            f,
            "| {} -> {}",
            lsn.kezdet_idopont.format("%H:%M"),
            lsn.veg_idopont.format("%H:%M")
        );
    }

    if let Some(tema) = &lsn.tema {
        _ = writeln!(&mut f, "| {tema}");
    }

    if lsn.absent() {
        _ = writeln!(&mut f, "| Ezen az órán nem voltál jelen.");
    }

    if lsn.cancelled() {
        _ = writeln!(&mut f, "| Ez az óra elmarad{}.", {
            if lsn.forecoming() {
                ""
            } else {
                "t"
            }
        });
    }

    if let Some(teacher) = &lsn.tanar_neve {
        // only show teacher, if there is no alternative one
        if lsn.helyettes_tanar_neve.is_none() {
            _ = write!(&mut f, "| {teacher}");
        }
    }

    if let Some(helyettes_tanar) = &lsn.helyettes_tanar_neve {
        _ = write!(&mut f, "| Helyettesítő tanár: {helyettes_tanar}");
    }

    f
}

/// get the number of days difference from today to `value`
/// if `value` is invalid, return `None`
fn day_diff(value: &str) -> Option<i8> {
    let value = value.to_lowercase();
    let num_from_mon = match value.as_str() {
        "hétfő" | "hé" | "mon" | "monday" => Some(1),
        "kedd" | "ke" | "tue" | "tuesday" => Some(2),
        "szerda" | "sze" | "wed" | "wednesday" => Some(3),
        "csütörtök" | "csüt" | "thu" | "thursday" => Some(4),
        "péntek" | "pé" | "fri" | "friday" => Some(5),
        _ => {
            warn!("\"{value}\" not valid as a day of week");
            None
        }
    };
    let today_as_num = Local::now().weekday().number_from_monday() as u8;
    info!("today_as_num: {today_as_num}");
    info!("other_as_num: {num_from_mon:?}");
    let diff = if let Some(from_mon) = num_from_mon {
        let x = from_mon as i8 - today_as_num as i8;
        if x < 0 {
            7 - today_as_num as i8 + x.abs() - 1
        } else {
            x
        }
    } else {
        match value.as_str() {
            "ma" | "today" => 0,
            "holnap" | "tomorrow" => 1,
            "tegnap" | "yesterday" => -1,
            _ => {
                warn!("\"{value}\" not valid as a day shifter word");
                return None;
            }
        }
    };

    info!("day diff: {diff}");
    Some(diff)
}

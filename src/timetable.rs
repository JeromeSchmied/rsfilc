//! lessons the student has

use crate::user::Usr;
use chrono::{Datelike, Duration, Local, NaiveDate};
use ekreta::{Lesson, Res};
use log::*;
use std::{fmt::Write, fs::File, io::Write as _, path::PathBuf};

pub fn handle(
    day: Option<NaiveDate>,
    user: &Usr,
    current: bool,
    out_p: Option<PathBuf>,
) -> Res<()> {
    let day = day.unwrap_or(Local::now().date_naive());
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
                nxt.nev
            );
        }
        for current_lesson in current_lessons {
            println!(
                "{}, {}m",
                current_lesson.nev,
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
/// # errors
///
/// - day shifter contains invalid number.
/// - any datetime is invalid.
pub fn parse_day(day: &str) -> Result<NaiveDate, String> {
    let today = Local::now().date_naive();
    let date = day.replace(['/', '.'], "-");
    info!("parsing date: {date}");

    // Parse From String
    let pfs = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d");
    if let Ok(ymd) = pfs(&date) {
        Ok(ymd)
    } else if let Ok(md) = pfs(&format!("{}-{date}", today.year())) {
        Ok(md)
    } else if let Ok(d) = pfs(&format!("{}-{}-{date}", today.year(), today.month())) {
        Ok(d)
    } else if date.starts_with('+') || date.ends_with('-') {
        info!("day_shift!");
        let day_shift = if date.starts_with('+') {
            info!("day_shift: +");
            date.parse::<i16>()
                .map_err(|e| format!("invalid +day shifter: {e:?}"))?
        } else {
            let date = &date[0..date.len() - 1];
            info!("day_shift: -");
            -date
                .parse::<i16>()
                .map_err(|e| format!("invalid day- shifter: {e:?}"))?
        };
        let day = today
            .checked_add_signed(Duration::days(day_shift.into()))
            .ok_or("invalid datetime")?;
        Ok(day)
    } else {
        Err(String::from("couldn't parse day specifier"))
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
    _ = write!(&mut f, "{}", lsn.nev);
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

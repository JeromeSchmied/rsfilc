//! lessons the student has

use crate::{fill, time::MyDate, user::Usr};
use chrono::{Datelike, Duration, Local, NaiveDate, TimeDelta};
use ekreta::{LDateTime, Lesson, Res};
use log::*;
use std::fmt::Write;

pub fn handle(day: Option<NaiveDate>, user: &Usr, current: bool, json: bool) -> Res<()> {
    let day = day.unwrap_or(default_day(user));
    let all_lessons_till_day = user.get_timetable(day, true)?;
    let lessons = user.get_timetable(day, false)?;
    if lessons.is_empty() {
        if json {
            println!("null");
        } else {
            println!("{day} ({}) nincs rögzített órád, juhé!", day.weekday());
        }
        return Ok(());
    }
    if current {
        if let Some(nxt) = next_lesson(&all_lessons_till_day) {
            if json {
                let data = serde_json::to_string(&(mins_till(nxt.kezdet_idopont), nxt))?;
                println!("{data}");
            } else {
                println!("{}m -> {}", mins_till(nxt.kezdet_idopont), nxt.nev);
            }
        }
        for cnt_lsn in current_lessons(&lessons) {
            if json {
                let data = serde_json::to_string(&(mins_till(cnt_lsn.veg_idopont), cnt_lsn))?;
                println!("{data}");
            } else {
                println!("{}, {}m", cnt_lsn.nev, mins_till(cnt_lsn.veg_idopont));
            }
        }
        return Ok(());
    }
    if json {
        let json = serde_json::to_string(&lessons)?;
        println!("{json}");
    } else {
        user.print_day(lessons);
    }
    Ok(())
}

/// minutes `till` now
fn mins_till(till: LDateTime) -> i64 {
    (till - Local::now()).num_minutes()
}
/// Parse the day got as `argument`.
/// # errors
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
/// # Warning
/// returns a `Vec<&Lesson>`, as a person might accidentally have more than one lessons at a time
pub fn current_lessons(lessons: &[Lesson]) -> Vec<&Lesson> {
    lessons.iter().filter(|lsn| lsn.happening()).collect()
}
/// Returns the next [`Lesson`] of this [`User`] from `lessons` which shall include today's [`Lesson`]s.
/// # Warning
/// There might accidentally be more next [`Lesson`]s. In this case only one of them is returned.
/// Also, if there is any `current_lesson`, None is returned
pub fn next_lesson(lessons: &[Lesson]) -> Option<&Lesson> {
    if !current_lessons(lessons).is_empty() {
        return None;
    }
    lessons
        .iter()
        .find(|lsn| lsn.forecoming() && !ignore_lesson(*lsn))
}
/// whether it's fake or cancelled
fn ignore_lesson(lsn: &Lesson) -> bool {
    lsn.kamu_smafu() || lsn.cancelled()
}

/// you may want to check `lsn` validity: `lsn.kamu_smafu()`
pub fn display(lsn: &Lesson) -> String {
    let mut f = String::new();
    _ = write!(&mut f, "{}", lsn.nev);
    if let Some(room) = &lsn.terem_neve {
        _ = writeln!(&mut f, ", {}", room.replace("terem", "").trim());
    }

    _ = writeln!(
        f,
        "| {} -> {}",
        lsn.kezdet_idopont.format("%H:%M"),
        lsn.veg_idopont.format("%H:%M")
    );

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

impl Usr {
    /// print all lessons of a day
    pub fn print_day(&self, mut lessons: Vec<Lesson>) {
        let Some(first_lesson) = lessons.first() else {
            return;
        };
        let day_start = first_lesson.kezdet_idopont;
        let header = if first_lesson.kamu_smafu() {
            format!("{}", lessons.remove(0).nev)
        } else {
            format!("{}, {}", day_start.hun_day_of_week(), day_start.pretty(),)
        };
        println!("{header}");
        fill(&header, '|', "");
        let tests = self.get_tests((None, None)).unwrap_or_default();
        let all_lessons_till_day = self
            .get_timetable(day_start.date_naive(), true)
            .unwrap_or_default();

        for (n, lsn) in lessons.iter().enumerate() {
            // calculate `n`. this lesson is
            let nth = lsn.oraszam.unwrap_or(u8::MAX);
            if n as u8 + 2 == nth
                && lessons
                    .get(n.overflowing_sub(1).0)
                    .is_none_or(|prev| prev.oraszam.unwrap_or(u8::MAX) == n as u8)
            {
                let no_lesson_buf = format!(
                        "\n\n{}. Lyukas (avagy Lukas) óra\n| Erre az időpontra nincsen tanóra rögzítve.",
                        n + 1
                    );
                println!("{no_lesson_buf}");
                fill(&no_lesson_buf, '^', "ojjé");
            }
            // so fill() works fine
            let mut printer = format!("\n\n{nth}. {}", display(lsn));

            if let Some(Some(test)) = lsn
                .bejelentett_szamonkeres_uid
                .as_ref()
                .map(|test_uid| tests.iter().find(|t| t.uid == *test_uid))
            {
                printer += &format!(
                    "\n| {}{}",
                    test.modja.leiras,
                    if let Some(topic) = test.temaja.as_ref() {
                        format!(": {topic}")
                    } else {
                        String::new()
                    }
                );
            }
            println!("{printer}");

            let (with, inlay_hint) = if lsn.happening() {
                ('$', format!("{} perc", mins_till(lsn.veg_idopont)))
            } else if next_lesson(&all_lessons_till_day).is_some_and(|nxt| nxt == lsn) {
                ('>', format!("{} perc", mins_till(lsn.kezdet_idopont)))
            } else if lsn.cancelled() {
                let past_morpheme = if lsn.forecoming() { "" } else { "t" };
                ('X', format!("elmarad{past_morpheme}"))
            } else {
                ('-', String::new())
            };
            fill(&printer, with, inlay_hint);
        }
    }
}

pub fn default_day(user: &Usr) -> NaiveDate {
    let now = Local::now();
    let today = now.date_naive();
    let end_of_today = if let Ok(mut lessons) = user.get_timetable(today, false) {
        lessons.retain(|l| !ignore_lesson(l));
        lessons.last().map(|l| l.veg_idopont)
    } else {
        return today;
    };

    let mut skip_days = TimeDelta::days(0);
    if end_of_today.is_none_or(|eot| eot < now) {
        skip_days = TimeDelta::days(1); // skipping today, as it's already done
        while let Ok(lsns) = user.get_timetable(today + skip_days, false) {
            if next_lesson(&lsns).is_some() {
                break;
            }
            skip_days += TimeDelta::days(1);
        }
    }
    today + skip_days
}

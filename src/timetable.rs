//! lessons the student has

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// Parse the day got as `argument`.
///
/// # Panics
///
/// Panics if
/// - day shifter contains invalid number.
/// - any datetime is invalid.
pub fn parse_day(day: &Option<String>) -> NaiveDate {
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
        } else if let Some(day) = day_as_num_from_str(&date) {
            /* eg.:
            today == thursday == 4
            looking_for == tuesday == 2
            (7 - today) + looking_for
            */
            let today_as_num = Local::now().weekday().number_from_monday() as u8;
            info!("today_as_num: {today_as_num}");
            let other_as_num = day;
            info!("other_as_num: {other_as_num}");
            let diff = 7 - today_as_num + other_as_num;
            info!("diff: {diff}");
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

/// endpoint
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/OrarendElemek"
}

/// Returns the current [`Lesson`]s of this [`User`] from `lessons` which shall include today's [`Lesson`]s.
///
/// # Warning
///
/// returns a `Vec<&Lesson>`, as a person might accidentally have more than one lessons at a time
pub fn current_lessons(lessons: &[Lesson]) -> Vec<&Lesson> {
    info!("searching for current lesson(s)");
    lessons
        .iter()
        .filter(|lsn| lsn.happening() && !lsn.cancelled())
        .collect()
}
/// Returns the next [`Lesson`] of this [`User`] from `lessons` which shall include today's [`Lesson`]s.
///
/// # Warning
///
/// There might accidentally be more next [`Lesson`]s. In this case only one of them is returned.
pub fn next_lesson(lessons: &[Lesson]) -> Option<&Lesson> {
    info!("searching for next lesson");
    lessons
        .iter()
        .filter(|lsn| lsn.forecoming() && !lsn.shite())
        .collect::<Vec<_>>()
        .first()
        .copied()
}

/// a lesson
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    // subject of the lesson
    #[serde(rename(deserialize = "Nev"))]
    pub subject: String,
    // room in which it will be held
    #[serde(rename(deserialize = "TeremNeve"))]
    room: Option<String>,

    // start datetime
    #[serde(rename(deserialize = "KezdetIdopont"))]
    start: String,
    // end datetime
    #[serde(rename(deserialize = "VegIdopont"))]
    end: String,

    /// topic of the lesson
    #[serde(rename(deserialize = "Tema"))]
    topic: Option<String>,

    /// name of the teacher
    #[serde(rename(deserialize = "TanarNeve"))]
    teacher: Option<String>,
    /// alternative teacher's name if any
    #[serde(rename(deserialize = "HelyettesTanarNeve"))]
    alt_teacher: Option<String>,

    /// subject: information about the type of the lesson: eg.: maths, history
    #[serde(rename(deserialize = "Tantargy"))]
    subject_details: Option<HashMap<String, Value>>,

    /// whether it has been cancelled or what
    #[serde(rename(deserialize = "Allapot"))]
    status: Option<HashMap<String, String>>,

    /// info about the student being present
    #[serde(rename(deserialize = "TanuloJelenlet"))]
    absence: Option<HashMap<String, String>>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}

impl Lesson {
    /// The two goddamn [`Lesson`]s should happen in the same time.
    pub fn same_time(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
    /// Returns whether this [`Lesson`] has been/will be cancelled.
    pub fn cancelled(&self) -> bool {
        self.status
            .as_ref()
            .is_some_and(|state| state.get("Nev").is_some_and(|state| state == "Elmaradt"))
    }
    /// Returns whether the student has appeared on this [`Lesson`].
    pub fn absent(&self) -> bool {
        self.absence.as_ref().is_some_and(|absence| {
            absence
                .get("Nev")
                .is_some_and(|presence| presence == "Hianyzas")
        })
    }
    /// Returns the start of this [`Lesson`].
    ///
    /// # Panics
    ///
    /// Panics if `kezdet_idopont` is invalid as date.
    pub fn start(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.start).unwrap().into()
    }
    /// Returns the end of this [`Lesson`].
    ///
    /// # Panics
    ///
    /// Panics if `veg_idopont` is invalid as date.
    pub fn end(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.end).unwrap().into()
    }

    /// Returns the subject id of this [`Lesson`].
    pub fn subject_id(&self) -> Option<String> {
        Some(
            self.subject_details
                .as_ref()?
                .get("Kategoria")?
                .get("Nev")?
                .to_string()
                .trim_matches('"')
                .to_string(),
        )
    }

    /// Returns whether this [`Lesson`] is currently happening.
    pub fn happening(&self) -> bool {
        self.start() <= Local::now() && self.end() >= Local::now()
    }

    /// Returns whether this [`Lesson`] is a forecoming one: to be done.
    pub fn forecoming(&self) -> bool {
        self.start() > Local::now()
    }

    /// Returns whether this [`Lesson`] is just false positive, meaning it's just a title for a day.
    pub fn shite(&self) -> bool {
        self.start().signed_duration_since(self.end()).is_zero()
    }
}
impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.subject)?;
        if let Some(room) = &self.room {
            writeln!(f, ", {}", room.replace("terem", "").trim())?;
        }

        if !self.shite() {
            writeln!(
                f,
                "| {} -> {}",
                self.start().format("%H:%M"),
                self.end().format("%H:%M")
            )?;
        }

        if let Some(tema) = &self.topic {
            writeln!(f, "| {tema}")?;
        }

        if self.absent() {
            writeln!(f, "| Ezen az órán nem voltál jelen.")?;
        }

        if self.cancelled() {
            writeln!(f, "| Ez az óra elmarad{}.", {
                if self.forecoming() {
                    ""
                } else {
                    "t"
                }
            })?;
        }

        if let Some(teacher) = &self.teacher {
            // only show teacher, if there is no alternative one
            if self.alt_teacher.is_none() {
                write!(f, "| {teacher}")?;
            }
        }

        if let Some(helyettes_tanar) = &self.alt_teacher {
            write!(f, "| Helyettesítő tanár: {helyettes_tanar}")?;
        }

        Ok(())
    }
}

/// get the `number_from_monday` from `value`
/// if day is invalid, return `None`
fn day_as_num_from_str(value: &str) -> Option<u8> {
    match value.to_lowercase().as_str() {
        "hétfő" | "hé" | "mon" | "monday" => Some(1),
        "kedd" | "ke" | "tue" | "tuesday" => Some(2),
        "szerda" | "sze" | "wed" | "wednesday" => Some(3),
        "csütörtök" | "csüt" | "thu" | "thursday" => Some(4),
        "péntek" | "pé" | "fri" | "friday" => Some(5),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

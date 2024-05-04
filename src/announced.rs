//! Announced tests

use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

use crate::MyDate;

/// endpoint
/// "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek"
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek"
}

/// announced test
#[derive(Debug, Deserialize)]
pub struct Ancd {
    /// date of doing test
    #[serde(rename(deserialize = "Datum"))]
    date: String,
    /// date of entry
    #[serde(rename(deserialize = "BejelentesDatuma"))]
    entry_date: String,

    /// teacher who entered it
    #[serde(rename(deserialize = "RogzitoTanarNeve"))]
    teacher_entered: String,

    /// nth lesson of that day
    #[serde(rename(deserialize = "OrarendiOraOraszama"))]
    pub nth: Option<u8>,

    /// name of the subject
    #[serde(rename(deserialize = "TantargyNeve"))]
    subject: String,
    /// subject: information about the type of the lesson: eg.: maths, history
    #[serde(rename(deserialize = "Tantargy"))]
    _subject_details: HashMap<String, Value>,

    /// topic of the test
    #[serde(rename(deserialize = "Temaja"))]
    pub topic: String,

    /// how it'll be done
    #[serde(rename(deserialize = "Modja"))]
    kind: HashMap<String, Value>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Ancd {
    /// Returns the entry date of this [`Announced`].
    ///
    /// # Panics
    ///
    /// Panics if data contains invalid date-time.
    pub fn entry_date(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.entry_date)
            .expect("invalid date-time")
            .into()
    }
    /// Returns the day when this [`Announced`] will be written by the student.
    ///
    /// # Panics
    ///
    /// Panics if data contains invalid date-time.
    pub fn day(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.date).unwrap().into()
    }
    /// Returns the kind of this [`Announced`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `kind`.
    pub fn kind(&self) -> String {
        self.kind
            .get("Leiras")
            .unwrap()
            .to_string()
            .trim_matches('"')
            .to_owned()
    }
    /// filter [`Ancd`] tests by `subj`ect
    pub fn filter_by_subject(ancds: &mut Vec<Ancd>, subj: &str) {
        info!("filtering announced tests by subject: {}", subj);
        ancds.retain(|ancd| ancd.subject.to_lowercase().contains(&subj.to_lowercase()));
    }
}
impl fmt::Display for Ancd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "| {}", &self.day().pretty())?;
        write!(f, " {}", self.subject)?;
        writeln!(f, ": {}", self.topic)?;

        writeln!(f, "| {}", self.kind())?;
        writeln!(f, "| {}", self.teacher_entered)?;
        write!(f, "| Rögzítés dátuma: {}", &self.entry_date().pretty())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

//! Announced tests

use chrono::{DateTime, Local};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt};

use crate::MyDate;

/// endpoint
/// "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek"
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek"
}

/// announced test
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Ancd {
    /// date of doing test
    #[serde(rename(deserialize = "Datum", serialize = "Datum"))]
    pub date: DateTime<Local>,
    /// date of entry
    #[serde(rename(deserialize = "BejelentesDatuma", serialize = "BejelentesDatuma"))]
    entry_date: DateTime<Local>,

    /// teacher who entered it
    #[serde(rename(deserialize = "RogzitoTanarNeve", serialize = "RogzitoTanarNeve"))]
    teacher_entered: String,

    /// nth lesson of that day
    #[serde(rename(deserialize = "OrarendiOraOraszama", serialize = "OrarendiOraOraszama"))]
    pub nth: Option<u8>,

    /// name of the subject
    #[serde(rename(deserialize = "TantargyNeve", serialize = "TantargyNeve"))]
    subject: String,
    /// subject: information about the type of the lesson: eg.: maths, history
    #[serde(rename(deserialize = "Tantargy", serialize = "Tantargy"))]
    _subject_details: HashMap<String, Value>,

    /// topic of the test
    #[serde(rename(deserialize = "Temaja", serialize = "Temaja"))]
    pub topic: Option<String>,

    /// how it'll be done
    #[serde(rename(deserialize = "Modja", serialize = "Modja"))]
    kind: HashMap<String, Value>,

    /// not needed
    #[serde(flatten)]
    #[serde(skip)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Ancd {
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
        write!(f, "| {}", &self.date.pretty())?;
        write!(f, " {}", self.subject)?;
        if let Some(tc) = &self.topic {
            write!(f, ": {}", tc)?;
        }

        writeln!(f, "\n| {}", self.kind())?;
        writeln!(f, "| {}", self.teacher_entered)?;
        write!(f, "| Rögzítés dátuma: {}", &self.entry_date.pretty())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

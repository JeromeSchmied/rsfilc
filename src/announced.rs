//! Announced tests

use crate::pretty_date;
use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

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
        write!(f, "| {}", self.kind())?;
        writeln!(f, ", {}", pretty_date(&self.day()))?;
        write!(f, "| {}", self.subject)?;
        writeln!(f, ": {}", self.topic)?;
        writeln!(f, "| {}", self.teacher_entered)?;
        write!(f, "| Rögzítés dátuma: {}", pretty_date(&self.entry_date()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let lesson_json = r#"{
        "Uid": "00000",
        "Datum": "2023-09-10T22:00:00Z",
        "BejelentesDatuma": "2023-09-08T13:35:07Z",
        "RogzitoTanarNeve": "Teszt Mónika",
        "OrarendiOraOraszama": 6,
        "Tantargy": {
            "Uid": "000000",
            "Nev": "matematika",
            "Kategoria": {
                "Uid": "0000,matematika",
                "Nev": "matematika",
                "Leiras": "Matematika"
            },
            "SortIndex": 0
        },
        "TantargyNeve": "matematika",
        "Temaja": "Matematikai logika",
        "Modja": {
            "Uid": "0000,irasbeli_ropdolgozat",
            "Nev": "irasbeli_ropdolgozat",
            "Leiras": "Írásbeli röpdolgozat"
        },
        "OsztalyCsoport": {
            "Uid": "000000"
        }
    }"#;

        let anc = serde_json::from_str::<Ancd>(lesson_json);

        assert!(anc.is_ok(), "{:?}", anc);
        let abs = anc.unwrap();

        assert_eq!(abs.teacher_entered, "Teszt Mónika");
        assert_eq!(abs.nth, Some(6));
        assert_eq!(abs.subject, "matematika");
        assert_eq!(abs.topic, "Matematikai logika");
        assert_eq!(abs.kind(), "Írásbeli röpdolgozat");
    }
}

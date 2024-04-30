//! lessons the student has

use chrono::{DateTime, Duration, Local, NaiveDate};
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
        if let Ok(ndate) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            ndate
        } else if date.starts_with('+') || date.ends_with('-') {
            let day_shift = if date.starts_with('+') {
                date.parse::<i64>().expect("invalid +day shifter")
            } else {
                let date = &date[0..date.len() - 1];
                -date.parse::<i64>().expect("invalid day- shifter")
            };
            Local::now()
                .checked_add_signed(Duration::days(day_shift))
                .expect("invalid datetime")
                .date_naive()
        } else {
            Local::now().date_naive()
        }
    } else {
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
        .filter(|lsn| lsn.happening() && lsn.cancelled())
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
        .filter(|lsn| lsn.forecoming())
        .collect::<Vec<_>>()
        .first()
        .copied()
}

/// a lesson
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    // name of the lesson
    nev: String,
    // room in which it will be held
    terem_neve: Option<String>,

    // start datetime
    kezdet_idopont: String,
    // end datetime
    veg_idopont: String,

    /// topic of the lesson
    tema: Option<String>,

    /// name of the teacher
    tanar_neve: Option<String>,
    /// alternative teacher's name if any
    helyettes_tanar_neve: Option<String>,

    /// subject: information about the type of the lesson: eg.: maths, history
    tantargy: Option<HashMap<String, Value>>,

    /// whether it has been cancelled or what
    allapot: Option<HashMap<String, String>>,

    /// info about the student being present
    tanulo_jelenlet: Option<HashMap<String, String>>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Lesson {
    /// Returns whether this [`Lesson`] has been/will be cancelled.
    pub fn cancelled(&self) -> bool {
        self.allapot
            .as_ref()
            .is_some_and(|state| state.get("Nev").is_some_and(|state| state == "Elmaradt"))
    }
    /// Returns whether the student has appeared on this [`Lesson`].
    pub fn absent(&self) -> bool {
        self.tanulo_jelenlet.as_ref().is_some_and(|absence| {
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
        DateTime::parse_from_rfc3339(&self.kezdet_idopont)
            .expect("couldn't parse kezdet_idopont")
            .into()
    }
    /// Returns the end of this [`Lesson`].
    ///
    /// # Panics
    ///
    /// Panics if `veg_idopont` is invalid as date.
    pub fn end(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.veg_idopont)
            .expect("couldn't parse veg_idopont")
            .into()
    }
    /// Returns the subject id of this [`Lesson`].
    pub fn subject_id(&self) -> Option<String> {
        Some(
            self.tantargy
                .as_ref()?
                .get("Kategoria")?
                .get("Nev")?
                .to_string()
                .trim_matches('"')
                .to_string(),
        )
    }
    /// Returns the subject id of this [`Lesson`].
    pub fn subject(&self) -> String {
        self.nev.clone()
    }

    /// Returns whether this [`Lesson`] is currently happening.
    pub fn happening(&self) -> bool {
        self.start() <= Local::now() && self.end() >= Local::now()
    }

    /// Returns whether this [`Lesson`] is a forecoming one: to be done.
    pub fn forecoming(&self) -> bool {
        self.start() > Local::now()
    }

    // pub fn nth_of_day(lessons: &[Lesson]) -> Option<Lesson> {
    //     todo!()
    // }
    // pub fn parse_time(time: &str) ->
}
impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.nev)?;
        if let Some(room) = &self.terem_neve {
            writeln!(f, "a(z) {} teremben", room.replace("terem", "").trim())?;
        } else {
            writeln!(f)?;
        }

        if let Some(tema) = &self.tema {
            writeln!(f, "Témája: {tema}")?;
        }

        if self.absent() {
            writeln!(f, "Ezen az órán nem voltál jelen.")?;
        }

        if self.cancelled() {
            writeln!(f, "Ez az óra elmaradt.")?;
        }

        if !self.start().signed_duration_since(self.end()).is_zero() {
            writeln!(
                f,
                "{} -> {}",
                self.start().time().format("%H:%M"),
                self.end().time().format("%H:%M")
            )?;
        }

        if let Some(teacher) = &self.tanar_neve {
            writeln!(f, "Tanár: {teacher}")?;
        }

        if let Some(helyettes_tanar) = &self.helyettes_tanar_neve {
            writeln!(f, "Helyettesítő tanár: {helyettes_tanar}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let lesson_json = r#"{
        "Uid": "XXXXXXX,TanitasiOra,2024-03-17T23:00:00Z",
        "Datum": "2024-03-17T23:00:00Z",
        "KezdetIdopont": "2024-03-18T08:50:00Z",
        "VegIdopont": "2024-03-18T09:35:00Z",
        "Nev": "fizika",
        "Oraszam": 3,
        "OraEvesSorszama": 72,
        "OsztalyCsoport": {
            "Uid": "837087",
            "Nev": "XX.X"
        },
        "TanarNeve": "Teszt Katalin",
        "Tantargy": {
            "Uid": "368813",
            "Nev": "fizika",
            "Kategoria": {
                "Uid": "1219,fizika",
                "Nev": "fizika",
                "Leiras": "Fizika"
            },
            "SortIndex": 0
        },
        "Tema": "Félvezetők",
        "TeremNeve": "Fizika",
        "Tipus": {
            "Uid": "2,TanitasiOra",
            "Nev": "TanitasiOra",
            "Leiras": "Tanítási óra"
        },
        "TanuloJelenlet": {
            "Uid": "XXXX,Jelenlet",
            "Nev": "Jelenlet",
            "Leiras": "A tanuló részt vett a tanórán"
        },
        "Allapot": {
            "Uid": "1,Naplozott",
            "Nev": "Naplozott",
            "Leiras": "Naplózott"
        },
        "HelyettesTanarNeve": null,
        "HaziFeladatUid": null,
        "FeladatGroupUid": null,
        "NyelviFeladatGroupUid": null,
        "BejelentettSzamonkeresUid": null,
        "IsTanuloHaziFeladatEnabled": false,
        "IsHaziFeladatMegoldva": false,
        "Csatolmanyok": [],
        "IsDigitalisOra": false,
        "DigitalisEszkozTipus": "Na",
        "DigitalisPlatformTipus": "Na",
        "DigitalisTamogatoEszkozTipusList": ["Na"],
        "Letrehozas": "2023-08-26T18:15:00",
        "UtolsoModositas": "2023-08-26T18:15:00"
    }"#;

        let lesson = serde_json::from_str::<Lesson>(lesson_json);

        assert!(lesson.is_ok(), "{:?}", lesson);
        let lesson = lesson.unwrap();

        assert_eq!(lesson.nev, "fizika");
        assert_eq!(lesson.terem_neve, Some("Fizika".to_string()));
        assert_eq!(lesson.tema, Some("Félvezetők".to_string()));
        assert_eq!(lesson.kezdet_idopont, "2024-03-18T08:50:00Z");
        assert_eq!(lesson.veg_idopont, "2024-03-18T09:35:00Z");
        assert_eq!(lesson.tanar_neve, Some("Teszt Katalin".to_string()));
        assert_eq!(lesson.helyettes_tanar_neve, None);
        assert!(!lesson.cancelled());
        assert!(!lesson.absent());
        assert_eq!(lesson.subject_id(), Some("fizika".to_string()));
    }
}

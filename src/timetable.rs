//! lessons the student has

use crate::{day_of_week, pretty_date};
use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// a lesson
#[derive(Debug, Deserialize)]
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
    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/OrarendElemek"
    }
    /// print all lessons of a day
    pub fn print_day(lessons: &[Lesson]) {
        if let Some(first_lesson) = lessons.first() {
            println!(
                "    {} ({})\n",
                pretty_date(&first_lesson.start()),
                day_of_week(
                    first_lesson
                        .start()
                        .weekday()
                        .number_from_monday()
                        .try_into()
                        .unwrap()
                )
            );
            for lesson in lessons {
                println!("{lesson}\n");
            }
        }
    }
    /// Returns whether this [`Lesson`] has been / will be cancelled.
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
    /// Returns the subject of this [`Lesson`].
    pub fn subject(&self) -> Option<String> {
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
    // pub fn parse_time(time: &str) ->
}
impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.nev)?;
        if let Some(room) = &self.terem_neve {
            writeln!(f, "a(z) {} teremben", room.replace("terem", ""))?;
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
        assert_eq!(lesson.subject(), Some("fizika".to_string()));
    }
}

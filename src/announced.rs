//! Announced tests

use crate::pretty_date;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// announced test
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ancd {
    /// date of doing test
    datum: String,
    /// date of entry
    bejelentes_datuma: String,

    /// teacher who entered it
    rogzito_tanar_neve: String,

    /// nth lesson of that day
    orarendi_ora_oraszama: Option<u8>,

    /// name of the subject
    tantargy_neve: String,
    /// subject: information about the type of the lesson: eg.: maths, history
    _tantargy: HashMap<String, Value>,

    /// topic of the test
    temaja: String,

    /// how it'll be done
    modja: HashMap<String, Value>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Ancd {
    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek"
    }
    /// Returns the entry date of this [`Announced`].
    ///
    /// # Panics
    ///
    /// Panics if data contains invalid date-time.
    pub fn entry_date(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.bejelentes_datuma)
            .expect("invalid date-time")
            .into()
    }
    /// Returns the day when this [`Announced`] will be written by the student.
    ///
    /// # Panics
    ///
    /// Panics if data contains invalid date-time.
    pub fn day(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.datum)
            .expect("invalid date-time")
            .into()
    }
    /// Returns the kind of this [`Announced`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `kind`.
    fn kind(&self) -> String {
        self.modja
            .get("Leiras")
            .expect("couldn't find kind")
            .to_string()
            .trim_matches('"')
            .to_owned()
    }
}
impl fmt::Display for Ancd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.tantargy_neve)?;
        writeln!(f, "{}", self.temaja)?;
        writeln!(f, "{}", self.kind())?;
        writeln!(f, "Írás dátuma: {}", pretty_date(&self.day()))?;
        writeln!(f, "{}", self.rogzito_tanar_neve)?;
        writeln!(f, "Bejelentés dátuma: {}", pretty_date(&self.entry_date()))?;

        writeln!(f, "\n---------------------------------\n")?;

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

        assert_eq!(abs.rogzito_tanar_neve, "Teszt Mónika");
        assert_eq!(abs.orarendi_ora_oraszama, Some(6));
        assert_eq!(abs.tantargy_neve, "matematika");
        assert_eq!(abs.temaja, "Matematikai logika");
        assert_eq!(abs.kind(), "Írásbeli röpdolgozat");
    }
}

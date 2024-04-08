use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

use crate::pretty_date;

/// Absence
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Abs {
    /// subject: information about the type of the lesson: eg.: maths, history
    tantargy: HashMap<String, Value>,

    /// lesson from, to it was held
    ora: HashMap<String, Value>,

    /// teacher who entered it
    rogzito_tanar_neve: String,

    /// minutes of being late
    keses_percben: Option<String>,
    /// whether it's already verified
    igazolas_allapota: String,
    /// type of verification
    igazolas_tipusa: HashMap<String, Value>,

    /// not needed
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}
impl Abs {
    pub fn start(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(
            self.ora
                .get("KezdoDatum")
                .expect("couldn't find starting date")
                .to_string()
                .trim_matches('"'),
        )
        .expect("invalid date-time")
        .into()
    }
    pub fn end(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(
            self.ora
                .get("VegDatum")
                .expect("couldn't find starting date")
                .to_string()
                .trim_matches('"'),
        )
        .expect("invalid date-time")
        .into()
    }
    fn verif(&self) -> bool {
        self.igazolas_allapota == "Igazolt"
    }
    fn subj(&self) -> String {
        self.tantargy
            .get("Nev")
            .expect("couldn't find subject")
            .to_string()
            .trim_matches('"')
            .to_string()
    }
}
impl fmt::Display for Abs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.rogzito_tanar_neve)?;
        writeln!(f, "{}", self.subj())?;
        writeln!(
            f,
            "{} -> {}",
            pretty_date(&self.start()),
            pretty_date(&self.end()),
        )?;

        if self.verif() {
            writeln!(f, "igazolt")?;
        } else {
            writeln!(f, "igazolatlan")?;
        }

        if let Some(late) = &self.keses_percben {
            writeln!(f, "Kestel {} percet", late)?;
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let lesson_json = r#"{
        "Uid": "21413485",
        "Tantargy": {
            "Uid": "368848",
            "Nev": "osztályfőnöki",
            "Kategoria": {
                "Uid": "1242,egyeb",
                "Nev": "egyeb",
                "Leiras": "Egyéb"
            },
            "SortIndex": 0
        },
        "Ora": {
            "KezdoDatum": "2023-09-01T06:55:00Z",
            "VegDatum": "2023-09-01T07:40:00Z",
            "Oraszam": 2
        },
        "Datum": "2023-08-31T22:00:00Z",
        "RogzitoTanarNeve": "Vondervisztné Kapor Ágnes",
        "Tipus": {
            "Uid": "1500,hianyzas",
            "Nev": "hianyzas",
            "Leiras": "Hiányzás"
        },
        "Mod": {
            "Uid": "1,Tanorai",
            "Nev": "Tanorai",
            "Leiras": "Tanórai mulasztás"
        },
        "KesesPercben": null,
        "KeszitesDatuma": "2023-09-02T08:09:19Z",
        "IgazolasAllapota": "Igazolt",
        "IgazolasTipusa": {
            "Uid": "6834,Kikero",
            "Nev": "Kikero",
            "Leiras": "Kikérő"
        },
        "OsztalyCsoport": {
            "Uid": "837087"
        }
    }"#;

        let lesson = serde_json::from_str::<Abs>(lesson_json);

        assert!(lesson.is_ok(), "{:?}", lesson);
        let lesson = lesson.unwrap();

        // assert_eq!(lesson.nev, "fizika");
        // assert_eq!(lesson.terem_neve, Some("Fizika".to_string()));
        // assert_eq!(lesson.tema, Some("Félvezetők".to_string()));
        // assert_eq!(lesson.kezdet_idopont, "2024-03-18T08:50:00Z");
        // assert_eq!(lesson.veg_idopont, "2024-03-18T09:35:00Z");
        // assert_eq!(lesson.tanar_neve, Some("Teszt Katalin".to_string()));
        // assert_eq!(lesson.helyettes_tanar_neve, None);
        // assert!(!lesson.cancelled());
        // assert!(!lesson.absent());
        // assert_eq!(lesson.subject(), Some("fizika".to_string()));
    }
}

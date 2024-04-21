//! Absences

use crate::pretty_date;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// Absence info
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
    // /// type of verification
    // igazolas_tipusa: HashMap<String, Value>,
    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Abs {
    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/Mulasztasok"
    }
    /// Returns the starting date of this [`Abs`].
    ///
    /// # Panics
    ///
    /// Panics if
    /// - data doesn't contain `starting date`.
    /// - which is invalid.
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
    /// Returns the end date of this [`Abs`].
    ///
    /// # Panics
    ///
    /// Panics if
    /// - data doesn't contain `end date`.
    /// - which is invalid.
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
    /// Returns whether the [`Abs`] has been verified.
    pub fn verif(&self) -> bool {
        self.igazolas_allapota == "Igazolt"
    }
    /// Returns the subject of the lesson which was missed in this [`Abs`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `subject`.
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

        writeln!(f, "\n----------------------\n")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let lesson_json = r#"{
        "Uid": "00000000",
        "Tantargy": {
            "Uid": "000000",
            "Nev": "osztályfőnöki",
            "Kategoria": {
                "Uid": "0000,egyeb",
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
        "RogzitoTanarNeve": "Teszt Lajos",
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
            "Uid": "0000,Kikero",
            "Nev": "Kikero",
            "Leiras": "Kikérő"
        },
        "OsztalyCsoport": {
            "Uid": "000000"
        }
    }"#;

        let abs = serde_json::from_str::<Abs>(lesson_json);

        assert!(abs.is_ok(), "{:?}", abs);
        let abs = abs.unwrap();

        assert_eq!(abs.subj(), "osztályfőnöki");
        assert_eq!(abs.keses_percben, None);
        assert_eq!(abs.rogzito_tanar_neve, "Teszt Lajos");
        assert_eq!(abs.igazolas_allapota, "Igazolt");
        assert!(abs.verif());
    }
}

//! Absences

use crate::*;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// endpoint
/// "/ellenorzo/V3/Sajat/Mulasztasok"
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/Mulasztasok"
}

/// Absence info
#[derive(Debug, Deserialize)]
pub struct Abs {
    /// subject: information about the type of the lesson: eg.: maths, history
    #[serde(rename(deserialize = "Tantargy"))]
    subject: HashMap<String, Value>,

    /// lesson from, to it was held
    #[serde(rename(deserialize = "Ora"))]
    lesson: HashMap<String, Value>,

    /// teacher who entered it
    #[serde(rename(deserialize = "RogzitoTanarNeve"))]
    teacher: String,

    /// minutes of being late
    #[serde(rename(deserialize = "KesesPercben"))]
    mins_late: Option<String>,
    /// whether it's already verified
    #[serde(rename(deserialize = "IgazolasAllapota"))]
    verification_status: String,
    // /// type of verification
    // #[serde(rename(deserialize = "igazolasTipusa"))]
    // igazolas_tipusa: HashMap<String, Value>,
    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Abs {
    /// Returns the starting date of this [`Abs`].
    ///
    /// # Panics
    ///
    /// Panics if
    /// - data doesn't contain `starting date`.
    /// - which is invalid.
    pub fn start(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(
            self.lesson
                .get("KezdoDatum")
                .unwrap()
                .to_string()
                .trim_matches('"'),
        )
        .unwrap()
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
            self.lesson
                .get("VegDatum")
                .unwrap()
                .to_string()
                .trim_matches('"'),
        )
        .unwrap()
        .into()
    }
    /// Returns whether the [`Abs`] has been verified.
    pub fn verified(&self) -> bool {
        self.verification_status == "Igazolt"
    }
    /// Returns the subject of the lesson which was missed in this [`Abs`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `subject`.
    fn subj(&self) -> String {
        self.subject
            .get("Nev")
            .expect("couldn't find subject")
            .to_string()
            .trim_matches('"')
            .to_string()
    }

    /// filter [`Abs`]ences by `subj`ect
    pub fn filter_by_subject(abss: &mut Vec<Abs>, subj: &str) {
        info!("filtering absences by subject: {}", subj);
        abss.retain(|abs| abs.subj().to_lowercase().contains(&subj.to_lowercase()));
    }
}
impl fmt::Display for Abs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.teacher)?;
        writeln!(f, "{}", self.subj())?;
        writeln!(
            f,
            "{} -> {}",
            pretty_date(&self.start()),
            pretty_date(&self.end()),
        )?;

        if self.verified() {
            writeln!(f, "igazolt")?;
        } else {
            writeln!(f, "igazolatlan")?;
        }

        if let Some(late) = &self.mins_late {
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
        assert_eq!(abs.mins_late, None);
        assert_eq!(abs.teacher, "Teszt Lajos");
        assert_eq!(abs.verification_status, "Igazolt");
        assert!(abs.verified());
    }
}

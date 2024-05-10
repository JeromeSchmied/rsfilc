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
        writeln!(f, "| {}", self.subj())?;
        writeln!(f, "| {}", self.teacher)?;
        writeln!(
            f,
            "| {} -> {}",
            &self.start().pretty(),
            &self.end().pretty(),
        )?;
        if let Some(late) = &self.mins_late {
            writeln!(f, "| Késtél {late} percet")?;
        }

        if self.verified() {
            write!(f, "| igazolt")?;
        } else {
            write!(f, "| igazolatlan")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;

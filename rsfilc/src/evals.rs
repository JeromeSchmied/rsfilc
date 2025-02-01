//! evaluations/grades the user recieved

use crate::*;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// endpoint
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/Ertekelesek"
}

/// evaluation/grade
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Eval {
    // /// the time it was saved to `Kréta`?
    // #[serde(rename(deserialize = "RogzitesDatuma"))]
    // date_saved: String,
    /// the time it was actually earned?
    #[serde(rename(deserialize = "KeszitesDatuma", serialize = "KeszitesDatuma"))]
    pub earned: DateTime<Local>,

    /// subject: information about the type of the lesson: eg.: maths, history
    #[serde(rename(deserialize = "Tantargy", serialize = "Tantargy"))]
    subject: Option<HashMap<String, Value>>,

    /// topic of the evaluation
    #[serde(rename(deserialize = "Tema", serialize = "Tema"))]
    topic: Option<String>,

    /// type of it
    #[serde(rename(deserialize = "Tipus", serialize = "Tipus"))]
    kind: Option<HashMap<String, String>>,

    /// type of it ?
    #[serde(rename(deserialize = "Mod", serialize = "Mod"))]
    another_kind: Option<HashMap<String, String>>,

    /// name of the teacher who made the evaluation
    #[serde(rename(deserialize = "ErtekeloTanarNeve", serialize = "ErtekeloTanarNeve"))]
    teacher: Option<String>,

    // /// type, again?
    // jelleg: String,
    /// with number (1,2,3,4,5)
    #[serde(rename(deserialize = "SzamErtek", serialize = "SzamErtek"))]
    as_num: Option<u8>,
    /// with text and number actually (Elégtelen(1), Elégséges(2), Közepes(3), Jó(4), Példás(5))
    #[serde(rename(deserialize = "SzovegesErtek", serialize = "SzovegesErtek"))]
    as_txt: String,

    /// weigth in % (about: 0-5000 ?)
    #[serde(rename(deserialize = "SulySzazalekErteke", serialize = "SulySzazalekErteke"))]
    weight_in_percent: Option<u16>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Eval {
    /// Returns the subject of this [`Eval`].
    /// Eg. `magyar_nyelv_es_irodalom`
    pub fn subject_id(&self) -> Option<String> {
        Some(
            self.subject
                .as_ref()?
                .get("Kategoria")?
                .get("Nev")?
                .to_string()
                .trim_matches('"')
                .to_string(),
        )
    }

    /// Returns the subject's name of this [`Eval`].
    /// Eg. "Magyar nyelv és irodalom"
    pub fn subject_name(&self) -> Option<String> {
        Some(
            self.subject
                .as_ref()?
                .get("Kategoria")?
                .get("Leiras")?
                .to_string()
                .trim_matches('"')
                .to_string(),
        )
    }

    /// Returns the kind of this [`Eval`].
    /// Eg. "Memoriter"
    fn kind(&self) -> Option<String> {
        Some(self.another_kind.as_ref()?.get("Leiras")?.to_owned())
    }

    /// Filter `evals` by `kind`
    pub fn filter_by_kind_or_title(evals: &mut Vec<Eval>, filter: &str) {
        let filter = filter.to_lowercase();
        info!("filtering evals by kind: {}", filter);
        evals.retain(|eval| {
            eval.kind()
                .is_some_and(|kd| kd.to_lowercase().contains(&filter))
                || eval
                    .topic
                    .as_ref()
                    .is_some_and(|t| t.to_lowercase().contains(&filter))
        });
    }

    /// Filter `evals` by `subject`
    pub fn filter_by_subject(evals: &mut Vec<Eval>, subj: &str) {
        info!("filtering evals by subject: {}", subj);
        evals.retain(|eval| {
            eval.subject_id()
                .is_some_and(|kd| kd.to_lowercase().contains(&subj.to_lowercase()))
                || eval
                    .subject_name()
                    .is_some_and(|kd| kd.to_lowercase().contains(&subj.to_lowercase()))
        });
    }

    /// Calculate average of `evals` and `ghosts` evals
    pub fn average(evals: &[Eval], ghosts: &[u8]) -> f32 {
        info!("calculating average for evals");
        let evals = evals
            .iter()
            .filter(|eval| !eval.end_year() && !eval.half_year() && eval.as_num.is_some());

        // filter it, so only valid grades retain
        let ghosts = ghosts.iter().filter(|g| *g > &0 && *g <= &5);

        let sum = evals.clone().fold(0., |sum, cur| {
            sum + cur.as_num.unwrap_or(0) as f32 * cur.multi_from_percent()
        }) + ghosts.clone().fold(0., |sum, num| sum + *num as f32);

        let count = evals
            .clone()
            .fold(0., |sum, cur| sum + cur.multi_from_percent())
            + ghosts.count() as f32;

        sum / count
    }

    /// Returns the multiplication value from percent of this [`Eval`].
    /// Eg. for 100% -> 1
    fn multi_from_percent(&self) -> f32 {
        self.weight_in_percent.unwrap_or(100) as f32 / 100.
    }

    /// Returns the type id of this [`Eval`].
    /// Eg. `evkozi_jegy_ertekeles`
    fn type_id(&self) -> Option<String> {
        Some(self.kind.as_ref()?.get("Nev")?.to_owned())
    }
    /// Returns whether this [`Eval`] is half year eval.
    fn half_year(&self) -> bool {
        self.type_id().is_some_and(|t| t == "felevi_jegy_ertekeles")
    }
    /// Returns whether this [`Eval`] is end of year eval.
    fn end_year(&self) -> bool {
        self.type_id().is_some_and(|t| t == "evvegi_jegy_ertekeles")
    }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "| ")?;
        if let Some(desc) = &self.topic {
            write!(f, "{desc}: ")?;
        }
        writeln!(f, "{}", self.as_txt)?;
        if let Some(subject) = self.subject_name() {
            writeln!(f, "| {subject}")?;
        }
        if let Some(kind) = &self.kind() {
            writeln!(f, "| {kind}")?;
        }
        if self.half_year() {
            writeln!(f, "| Félévi értékelés")?;
        }
        if self.end_year() {
            writeln!(f, "| Év végi értékelés")?;
        }
        if let Some(teacher) = &self.teacher {
            writeln!(f, "| {teacher}")?;
        }
        write!(f, "| Időpont: {}", &self.earned.pretty())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

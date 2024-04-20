//! evaluations/grades the user recieved

use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// evaluation/grade
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Eval {
    // /// the time it was saved to `Kréta`
    // rogzites_datuma: String,
    /// the time it was actually earned
    keszites_datuma: String,

    /// subject: information about the type of the lesson: eg.: maths, history
    tantargy: Option<HashMap<String, Value>>,

    /// topic of the evaluation
    tema: Option<String>,

    /// type of it
    tipus: Option<HashMap<String, String>>,

    /// type of it ?
    r#mod: Option<HashMap<String, String>>,

    /// name of the teacher who made the evaluation
    ertekelo_tanar_neve: Option<String>,

    // /// type, again?
    // jelleg: String,
    /// with number (1,2,3,4,5)
    szam_ertek: Option<u8>,
    /// with text and number actually (Elégtelen(1), Elégséges(2), Közepes(3), Jó(4), Példás(5))
    szoveges_ertek: String,

    /// weigth in % (about: 0-5000 ?)
    suly_szazalek_erteke: Option<u16>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Eval {
    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/Ertekelesek"
    }

    /// Returns the subject of this [`Eval`].
    /// Eg. "magyar_nyelv_es_irodalom"
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

    /// Returns the subject's name of this [`Eval`].
    /// Eg. "Magyar nyelv és irodalom"
    pub fn subject_name(&self) -> Option<String> {
        Some(
            self.tantargy
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
        Some(self.r#mod.as_ref()?.get("Leiras")?.to_owned())
    }

    /// Returns the date when earned of this [`Eval`].
    ///
    /// # Panics
    ///
    /// Panics if `keszites_datuma` is invalid date-time.
    pub fn earned(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.keszites_datuma)
            .expect("couldn't parse veg_idopont")
            .into()
    }

    /// Filter `evals` by `kind`
    pub fn filter_by_kind(evals: &mut Vec<Eval>, kind: &str) {
        info!("filtering evals by kind: {}", kind);
        evals.retain(|eval| {
            eval.kind()
                .is_some_and(|kd| kd.to_lowercase().contains(&kind.to_lowercase()))
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

    /// Calculate average of `evals`
    pub fn average(evals: &[Eval]) -> f32 {
        info!("calculating average for evals");
        let evals = evals.iter().filter(|eval| {
            !eval
                .type_id()
                .is_some_and(|t| t.contains("felevi") || t.contains("evvegi"))
        });

        let sum: u16 = evals.clone().fold(0, |sum, cur| {
            sum + cur.szam_ertek.unwrap_or(0) as u16 * cur.multi_from_percent() as u16
        });

        let count = evals
            .clone()
            .fold(0, |sum, cur| sum + cur.multi_from_percent() as u16);

        sum as f32 / count as f32
    }

    /// Returns the multiplication value from percent of this [`Eval`].
    /// Eg. for 100% -> 1
    fn multi_from_percent(&self) -> u8 {
        (self.suly_szazalek_erteke.unwrap_or(100) / 100) as u8
    }

    /// Returns the type id of this [`Eval`].
    /// Eg. "evkozi_jegy_ertekeles"
    fn type_id(&self) -> Option<String> {
        Some(self.tipus.as_ref()?.get("Nev")?.to_owned())
    }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(desc) = &self.tema {
            write!(f, "{desc}: ")?;
        }
        writeln!(f, "{}", self.szoveges_ertek)?;
        if let Some(subject) = self.subject_name() {
            writeln!(f, "Tantárgy: {subject}")?;
        }
        if let Some(teacher) = &self.ertekelo_tanar_neve {
            writeln!(f, "Értékelő tanár: {teacher}")?;
        }

        if let Some(kind) = &self.kind() {
            writeln!(f, "Típus: {kind}")?;
        }
        writeln!(f, "Szertevés dátuma: {}", self.earned().format("%Y/%m/%d"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let eval_json = r#"{
        "Uid": "00000000,Ertekeles",
        "RogzitesDatuma": "2024-01-16T23:00:00Z",
        "KeszitesDatuma": "2024-01-18T16:48:53Z",
        "LattamozasDatuma": null,
        "Tantargy": {
            "Uid": "368800",
            "Nev": "irodalom",
            "Kategoria": {
                "Uid": "1198,magyar_nyelv_es_irodalom",
                "Nev": "magyar_nyelv_es_irodalom",
                "Leiras": "Magyar nyelv és irodalom"
            },
            "SortIndex": 3
        },
        "Tema": "Villon",
        "Tipus": {
            "Uid": "1518,evkozi_jegy_ertekeles",
            "Nev": "evkozi_jegy_ertekeles",
            "Leiras": "Évközi jegy/értékelés"
        },
        "Mod": {
            "Uid": "000000,AdatszotarElem",
            "Nev": "AdatszotarElem",
            "Leiras": "Memoriter"
        },
        "ErtekFajta": {
            "Uid": "1,Osztalyzat",
            "Nev": "Osztalyzat",
            "Leiras": "Elégtelen (1) és Jeles (5) között az öt alapértelmezett érték"
        },
        "ErtekeloTanarNeve": "Teszt Tamás",
        "Jelleg": "Ertekeles",
        "SzamErtek": 5,
        "SzovegesErtek": "Jeles(5)",
        "SulySzazalekErteke": 100,
        "SzovegesErtekelesRovidNev": null,
        "OsztalyCsoport": {
            "Uid": "837087"
        },
        "SortIndex": 3
    }"#;

        let eval = serde_json::from_str::<Eval>(eval_json);
        assert!(eval.is_ok());

        let eval = eval.unwrap();

        assert_eq!(eval.tema, Some("Villon".to_string()));
        assert_eq!(eval.ertekelo_tanar_neve, Some("Teszt Tamás".to_owned()));
        assert_eq!(eval.szam_ertek, Some(5));
        assert_eq!(eval.szoveges_ertek, "Jeles(5)");
        assert_eq!(eval.suly_szazalek_erteke, Some(100));
        assert_eq!(
            eval.subject_id(),
            Some("magyar_nyelv_es_irodalom".to_owned())
        );
        assert_eq!(
            eval.subject_name(),
            Some("Magyar nyelv és irodalom".to_owned())
        );
        assert_eq!(eval.kind(), Some("Memoriter".to_owned()));
        assert_eq!(eval.multi_from_percent(), 1);
        assert_eq!(eval.type_id(), Some("evkozi_jegy_ertekeles".to_owned()))
    }
}

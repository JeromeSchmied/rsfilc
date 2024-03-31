//! evaluations/grades the user recieved

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Eval {
    /// the time it was saved to `Kréta`
    rogzites_datuma: String,
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

    /// type, again?
    jelleg: String,

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
    /// Returns the subject of this [`Eval`].
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

    /// Returns the subject's name of this [`Eval`].
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
            .expect("coudln't parse veg_idopont")
            .into()
    }
}
impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(subject) = self.subject_name() {
            writeln!(f, "Tantárgy: {subject}")?;
        }
        if let Some(teacher) = &self.ertekelo_tanar_neve {
            writeln!(f, "Értékelő tanár: {teacher}")?;
        }

        writeln!(f, "Értékelés: {}", self.szoveges_ertek)?;

        if let Some(desc) = &self.tema {
            writeln!(f, "Leírás: {desc}")?;
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
    }
}

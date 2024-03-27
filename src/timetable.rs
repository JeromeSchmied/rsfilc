use crate::AnyErr;
use serde::Deserialize;
use speedate::DateTime;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    // ora neve
    nev: String,
    // terem
    terem_neve: String,

    // datetime
    kezdet_idopont: String,
    // datetime
    veg_idopont: String,

    /// topic of the lesson
    tema: Option<String>,

    /// name of the teacher
    tanar_neve: String,
    /// alternative teacher's name if any
    helyettes_tanar_neve: Option<String>,

    /// not needed
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}
impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} óra a {} teremben", self.nev, self.terem_neve)?;

        if let Some(tema) = &self.tema {
            writeln!(f, "Témája: {}", tema)?;
        }

        writeln!(f, "{} -> {}", self.kezdet_idopont, self.veg_idopont)?;
        writeln!(f, "Tanár: {}", self.tanar_neve)?;

        if let Some(helyettes_tanar) = &self.helyettes_tanar_neve {
            writeln!(f, "helyettes tanar: {:?}", helyettes_tanar)?;
        }

        Ok(())
    }
}
impl Lesson {
    pub fn print_day(lessons: Vec<Lesson>) {
        for lesson in lessons {
            println!("{}\n", lesson);
        }
    }
    pub fn tol(&self) -> DateTime {
        DateTime::from_str(&self.kezdet_idopont).expect("invalid date-time")
    }
    pub fn ig(&self) -> DateTime {
        DateTime::from_str(&self.veg_idopont).expect("invalid date-time")
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

        assert!(lesson.is_ok());
    }
}

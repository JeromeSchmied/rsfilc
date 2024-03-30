use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    // ora neve
    nev: String,
    // terem
    terem_neve: Option<String>,

    // datetime
    kezdet_idopont: String,
    // datetime
    veg_idopont: String,

    /// topic of the lesson
    tema: Option<String>,

    /// name of the teacher
    tanar_neve: Option<String>,
    /// alternative teacher's name if any
    helyettes_tanar_neve: Option<String>,

    /// whether it has been cancelled or what
    allapot: Option<HashMap<String, String>>,

    /// info about the student being present
    tanulo_jelenlet: Option<HashMap<String, String>>,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Lesson {
    pub fn print_day(lessons: Vec<Lesson>) {
        if let Some(first_lesson) = lessons.first() {
            println!(
                "{}({})\n",
                first_lesson.start().date_naive(),
                first_lesson.start().weekday()
            );
        }
        for lesson in lessons {
            println!("{}\n", lesson);
        }
    }
    pub fn from(&self) -> DateTime<Local> {
        DateTime::from_str(&self.kezdet_idopont).expect("invalid date-time")
    }
    pub fn to(&self) -> DateTime<Local> {
        DateTime::from_str(&self.veg_idopont).expect("invalid date-time")
    }
    pub fn cancelled(&self) -> bool {
        self.allapot
            .as_ref()
            .is_some_and(|state| state.get("Nev").is_some_and(|state| state == "Elmaradt"))
    }
    pub fn absent(&self) -> bool {
        self.tanulo_jelenlet.as_ref().is_some_and(|absence| {
            absence
                .get("Nev")
                .is_some_and(|presence| presence == "Hianyzas")
        })
    }
    pub fn start(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.kezdet_idopont)
            .expect("coudln't parse kezdet_idopont")
            .into()
    }
    pub fn end(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&self.veg_idopont)
            .expect("coudln't parse veg_idopont")
            .into()
    }
    // pub fn parse_time(time: &str) ->
}
impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} óra ", self.nev)?;
        if let Some(room) = &self.terem_neve {
            writeln!(f, "a(z) {} teremben", room)?;
        } else {
            writeln!(f)?;
        }

        if let Some(tema) = &self.tema {
            writeln!(f, "Témája: {}", tema)?;
        }

        if self.cancelled() {
            writeln!(f, "This lesson was cancelled")?;
        }

        writeln!(f, "{} -> {}", self.start().time(), self.end().time())?;

        if let Some(teacher) = &self.tanar_neve {
            writeln!(f, "Tanár: {}", teacher)?;
        }

        if let Some(helyettes_tanar) = &self.helyettes_tanar_neve {
            writeln!(f, "Helyettesítő tanár: {:?}", helyettes_tanar)?;
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

        assert!(lesson.is_ok());
    }
}

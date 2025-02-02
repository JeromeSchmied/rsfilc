use super::Endpoint;
use crate::{
    types::{OsztalyCsoport, Rektip, Tantargy},
    LDateTime, OptIrval,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Evaluation {
    pub uid: String,
    pub rogzites_datuma: LDateTime,
    pub keszites_datuma: LDateTime,
    pub lattamozas_datuma: Option<LDateTime>,
    pub tantargy: Tantargy,
    pub tema: Option<String>,
    pub tipus: Rektip,
    pub r#mod: Option<Rektip>,
    pub ertek_fajta: Rektip,
    pub ertekelo_tanar_neve: Option<String>,
    pub jelleg: String,
    pub szam_ertek: Option<u8>,
    pub szoveges_ertek: String,
    pub suly_szazalek_erteke: Option<u16>,
    pub szoveges_ertekeles_rovid_nev: Option<String>,
    pub osztaly_csoport: OsztalyCsoport,
    pub sort_index: i64,
}
impl Evaluation {
    /// Returns the multiplication value from percent of this [`Eval`].
    /// Eg. for 100% -> 1
    pub fn szorzo(&self) -> f32 {
        self.suly_szazalek_erteke.unwrap_or(100) as f32 / 100.
    }
    /// Returns whether this [`Eval`] is half year eval.
    pub fn felevi(&self) -> bool {
        self.tipus.nev == "felevi_jegy_ertekeles"
    }
    /// Returns whether this [`Eval`] is end of year eval.
    pub fn evvegi(&self) -> bool {
        self.tipus.nev == "evvegi_jegy_ertekeles"
    }
}
impl Endpoint for Evaluation {
    type QueryInput = OptIrval;

    fn path() -> &'static str {
        "/ellenorzo/V3/Sajat/Ertekelesek"
    }

    fn query(input: &Self::QueryInput) -> anyhow::Result<impl Serialize> {
        let mut q = vec![];
        if let Some(from) = input.0 {
            q.push(("datumTol", from.to_string()));
        }
        if let Some(to) = input.1 {
            q.push(("datumIg", to.to_string()));
        }
        Ok(q)
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluation;

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

        let eval = serde_json::from_str::<Evaluation>(eval_json);
        assert!(eval.is_ok());

        let eval = eval.unwrap();

        assert_eq!(eval.tema, Some("Villon".to_string()));
        assert_eq!(eval.ertekelo_tanar_neve, Some("Teszt Tamás".to_owned()));
        assert_eq!(eval.szam_ertek, Some(5));
        assert_eq!(eval.szoveges_ertek, "Jeles(5)");
        assert_eq!(eval.suly_szazalek_erteke, Some(100));
        assert_eq!(eval.tantargy.kategoria.nev, "magyar_nyelv_es_irodalom");
        assert_eq!(eval.tantargy.kategoria.leiras, "Magyar nyelv és irodalom");
        assert_eq!(eval.r#mod.as_ref().unwrap().leiras, "Memoriter");
        assert_eq!(eval.szorzo(), 1.);
        assert_eq!(eval.tipus.nev, "evkozi_jegy_ertekeles")
    }
}

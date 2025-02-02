use crate::{types::*, Endpoint, LDateTime};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    pub uid: String,
    pub datum: String,
    pub kezdet_idopont: LDateTime,
    pub veg_idopont: LDateTime,
    pub nev: String,
    pub oraszam: i64,
    pub ora_eves_sorszama: i64,
    pub osztaly_csoport: Uid,
    pub tanar_neve: Option<String>,
    pub tantargy: Tantargy,
    pub tema: Option<String>,
    pub terem_neve: Option<String>,
    pub tipus: Rektip,
    pub tanulo_jelenlet: Option<Rektip>,
    pub allapot: Option<Rektip>,
    pub helyettes_tanar_neve: Option<String>,
    pub hazi_feladat_uid: Option<String>,
    pub feladat_group_uid: Option<String>,
    pub nyelvi_feladat_group_uid: Option<String>,
    pub bejelentett_szamonkeres_uid: Option<String>,
    pub is_tanulo_hazi_feladat_enabled: bool,
    pub is_hazi_feladat_megoldva: bool,
    pub csatolmanyok: Vec<String>,
    pub is_digitalis_ora: bool,
    pub digitalis_eszkoz_tipus: Option<String>,
    pub digitalis_platform_tipus: Option<String>,
    pub digitalis_tamogato_eszkoz_tipus_list: Vec<String>,
    pub letrehozas: String,
    pub utolso_modositas: chrono::NaiveDateTime,
}
impl Lesson {
    /// The two goddamn [`Lesson`]s should happen in the same time.
    pub fn same_time(&self, other: &Self) -> bool {
        self.kezdet_idopont == other.kezdet_idopont && self.veg_idopont == other.veg_idopont
    }
    /// Returns whether this [`Lesson`] has been/will be cancelled.
    pub fn cancelled(&self) -> bool {
        self.allapot.as_ref().is_some_and(|a| a.nev == "Elmaradt")
    }
    /// Returns whether the student has appeared on this [`Lesson`].
    pub fn absent(&self) -> bool {
        self.tanulo_jelenlet
            .as_ref()
            .is_some_and(|absence| absence.nev == "Hianyzas")
    }
    /// Returns the subject id of this [`Lesson`].
    pub fn subject_id(&self) -> &String {
        &self.tantargy.kategoria.nev
    }
    /// Returns whether this [`Lesson`] is just false positive, meaning it's just a title for a day.
    pub fn kamu_smafu(&self) -> bool {
        self.kezdet_idopont
            .signed_duration_since(self.veg_idopont)
            .is_zero()
    }
    /// Returns whether this [`Lesson`] is currently happening.
    pub fn happening(&self) -> bool {
        if self.cancelled() {
            return false;
        }
        self.kezdet_idopont <= Local::now() && self.veg_idopont >= Local::now()
    }

    /// Returns whether this [`Lesson`] is a forecoming one: to be done.
    pub fn forecoming(&self) -> bool {
        self.kezdet_idopont > Local::now()
    }
}

impl Endpoint for Lesson {
    type QueryInput = (LDateTime, LDateTime);

    fn path() -> &'static str {
        "/ellenorzo/V3/Sajat/OrarendElemek"
    }

    fn query(input: &Self::QueryInput) -> anyhow::Result<impl Serialize> {
        let mut q = vec![];
        q.push(("datumTol", input.0.to_string()));
        q.push(("datumIg", input.1.to_string()));
        Ok(q)
    }
}

#[cfg(test)]
#[test]
fn works() {
    let lesson_json = r#"{ "Uid": "XXXXXXX,TanitasiOra,2024-03-17T23:00:00Z", "Datum": "2024-03-17T23:00:00Z", "KezdetIdopont": "2024-03-18T08:50:00Z", "VegIdopont": "2024-03-18T09:35:00Z", "Nev": "fizika", "Oraszam": 3, "OraEvesSorszama": 72, "OsztalyCsoport": { "Uid": "837087", "Nev": "XX.X" }, "TanarNeve": "Teszt Katalin", "Tantargy": { "Uid": "368813", "Nev": "fizika", "Kategoria": { "Uid": "1219,fizika", "Nev": "fizika", "Leiras": "Fizika" }, "SortIndex": 0 }, "Tema": "Félvezetők", "TeremNeve": "Fizika", "Tipus": { "Uid": "2,TanitasiOra", "Nev": "TanitasiOra", "Leiras": "Tanítási óra" }, "TanuloJelenlet": { "Uid": "XXXX,Jelenlet", "Nev": "Jelenlet", "Leiras": "A tanuló részt vett a tanórán" }, "Allapot": { "Uid": "1,Naplozott", "Nev": "Naplozott", "Leiras": "Naplózott" }, "HelyettesTanarNeve": null, "HaziFeladatUid": null, "FeladatGroupUid": null, "NyelviFeladatGroupUid": null, "BejelentettSzamonkeresUid": null, "IsTanuloHaziFeladatEnabled": false, "IsHaziFeladatMegoldva": false, "Csatolmanyok": [], "IsDigitalisOra": false, "DigitalisEszkozTipus": "Na", "DigitalisPlatformTipus": "Na", "DigitalisTamogatoEszkozTipusList": ["Na"], "Letrehozas": "2023-08-26T18:15:00", "UtolsoModositas": "2023-08-26T18:15:00" }"#;

    let lesson = serde_json::from_str::<Lesson>(lesson_json);

    assert!(lesson.is_ok(), "{:?}", lesson);
    let lesson = lesson.unwrap();

    assert_eq!(lesson.tantargy.nev, "fizika");
    assert_eq!(lesson.terem_neve, Some("Fizika".to_string()));
    assert_eq!(lesson.tema, Some("Félvezetők".to_string()));
    // assert_eq!(lesson.start, "2024-03-18T08:50:00Z");
    // assert_eq!(lesson.end, "2024-03-18T09:35:00Z");
    assert_eq!(lesson.tanar_neve, Some("Teszt Katalin".to_string()));
    assert_eq!(lesson.helyettes_tanar_neve, None);
    assert!(!lesson.cancelled());
    assert!(!lesson.absent());
    assert_eq!(lesson.subject_id().as_str(), "fizika");
    assert!(!lesson.kamu_smafu());
}

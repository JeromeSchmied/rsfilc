use crate::LDateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tantargy {
    pub uid: String,
    pub nev: String,
    pub kategoria: Rektip,
    pub sort_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rektip {
    pub uid: String,
    pub nev: String,
    pub leiras: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ora {
    pub kezdo_datum: LDateTime,
    pub veg_datum: LDateTime,
    pub oraszam: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Uid {
    pub uid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gondviselo {
    pub uid: String,
    pub idp_unique_id: String,
    pub nev: String,
    pub email_cim: String,
    pub telefonszam: String,
    pub is_torvenyes_kepviselo: bool,
    pub is_nincs_felugyeleti_joga: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bankszamla {
    pub bankszamla_szam: String,
    pub bankszamla_tulajdonos_tipus_id: Option<String>,
    pub bankszamla_tulajdonos_neve: Option<String>,
    pub is_read_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Intezmeny {
    pub uid: String,
    pub teljes_nev: String,
    pub rovid_nev: String,
    pub rendszermodulok: Vec<Rendszermodulok>,
    pub testreszabas_beallitasok: TestreszabasBeallitasok,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rendszermodulok {
    pub is_aktiv: bool,
    pub tipus: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestreszabasBeallitasok {
    pub is_diak_rogzithet_hazi_feladatot: bool,
    pub is_tanorak_temaja_megtekintheto_ellenorzoben: bool,
    pub is_osztaly_atlag_megjelenitese_ellenorzoben: bool,
    pub is_elerhetoseg_szerkesztheto: bool,
    pub ertekelesek_megjelenitesenek_kesleltetesenek_merteke: i64,
    pub kovetkezo_telepites_datuma: LDateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tagsag {
    pub besorolas_datuma: LDateTime,
    pub kisorolas_datuma: LDateTime,
}

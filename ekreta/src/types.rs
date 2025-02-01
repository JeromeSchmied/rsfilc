use chrono::{DateTime, Local};
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
    pub kezdo_datum: DateTime<Local>,
    pub veg_datum: DateTime<Local>,
    pub oraszam: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OsztalyCsoport {
    pub uid: String,
}

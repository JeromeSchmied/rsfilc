use super::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub id: String,
    pub azonosito: String,
    pub nev: String,
    pub rovid_nev: Option<String>,
    pub om_kod: String,
    pub kreta_link: String,
    pub telepules: String,
    pub aktiv_tanev_id: i64,
    pub aktiv_tanev_guid: String,
    pub aktiv_tanev_nev: String,
    pub kornyezet_id: i64,
    pub kornyezet_nev: String,
    pub kornyezet_teljes_nev: String,
    pub fenntarto_azonosito: String,
    pub fenntarto_nev: String,
}

impl Endpoint for School {
    type Args = ();

    fn path(_args: &Self::Args) -> String {
        "/intezmenyek/kreta/publikus".into()
    }

    fn base_url(_args: impl AsRef<str>) -> crate::Str {
        "https://kretaglobalapi.e-kreta.hu".into()
    }
}

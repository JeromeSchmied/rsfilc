use super::Endpoint;
use crate::types::{Rektip, Tagsag, Uid};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub uid: String,
    pub nev: String,
    pub osztaly_fonok: Uid,
    pub osztaly_fonok_helyettes: Option<Uid>,
    pub oktatas_nevelesi_feladat: Rektip,
    pub oktatas_nevelesi_kategoria: Rektip,
    pub oktatas_nevelesi_feladat_sort_index: i64,
    pub is_aktiv: bool,
    pub tipus: String,
    pub tagsagok: Vec<Tagsag>,
}

impl Endpoint for Class {
    type QueryInput = ();

    fn path(_args: impl AsRef<str>) -> String {
        "/ellenorzo/V3/Sajat/OsztalyCsoportok".into()
    }
}

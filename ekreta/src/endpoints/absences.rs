use crate::{types::*, Endpoint, Interval, Result, Str};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub uid: String,
    pub tantargy: Tantargy,
    pub ora: Ora,
    pub datum: DateTime<Local>,
    pub rogzito_tanar_neve: String,
    pub tipus: Rektip,
    pub r#mod: Rektip,
    pub keses_percben: Option<u8>,
    pub keszites_datuma: DateTime<Local>,
    pub igazolas_allapota: String,
    pub igazolas_tipusa: Rektip,
    pub osztaly_csoport: OsztalyCsoport,
}

impl Endpoint for Absence {
    type QueryInput = Interval;

    fn path() -> Str {
        "/ellenorzo/V3/Sajat/Mulasztasok".into()
    }

    fn query(input: &Self::QueryInput) -> Result<Option<impl Serialize>> {
        Ok(Some(input))
    }
}

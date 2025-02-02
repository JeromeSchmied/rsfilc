use crate::types::{Ora, OsztalyCsoport, Rektip, Tantargy};
use crate::LDateTime;
use crate::{Endpoint, OptIrval, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub uid: String,
    pub tantargy: Tantargy,
    pub ora: Ora,
    pub datum: LDateTime,
    pub rogzito_tanar_neve: String,
    pub tipus: Rektip,
    pub r#mod: Rektip,
    pub keses_percben: Option<u8>,
    pub keszites_datuma: LDateTime,
    pub igazolas_allapota: String,
    pub igazolas_tipusa: Rektip,
    pub osztaly_csoport: OsztalyCsoport,
}
impl Absence {
    /// Returns whether the [`Abs`] has been verified.
    pub fn igazolt(&self) -> bool {
        self.igazolas_allapota == "Igazolt"
    }
}

impl Endpoint for Absence {
    type QueryInput = OptIrval;

    fn path() -> &'static str {
        "/ellenorzo/V3/Sajat/Mulasztasok"
    }

    fn query(input: &Self::QueryInput) -> Result<impl Serialize> {
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
#[test]
fn works() {
    let lesson_json = r#"{ "Uid": "00000000", "Tantargy": { "Uid": "000000", "Nev": "osztályfőnöki", "Kategoria": { "Uid": "0000,egyeb", "Nev": "egyeb", "Leiras": "Egyéb" }, "SortIndex": 0 }, "Ora": { "KezdoDatum": "2023-09-01T06:55:00Z", "VegDatum": "2023-09-01T07:40:00Z", "Oraszam": 2 }, "Datum": "2023-08-31T22:00:00Z", "RogzitoTanarNeve": "Teszt Lajos", "Tipus": { "Uid": "1500,hianyzas", "Nev": "hianyzas", "Leiras": "Hiányzás" }, "Mod": { "Uid": "1,Tanorai", "Nev": "Tanorai", "Leiras": "Tanórai mulasztás" }, "KesesPercben": null, "KeszitesDatuma": "2023-09-02T08:09:19Z", "IgazolasAllapota": "Igazolt", "IgazolasTipusa": { "Uid": "0000,Kikero", "Nev": "Kikero", "Leiras": "Kikérő" }, "OsztalyCsoport": { "Uid": "000000" } }"#;

    let abs = serde_json::from_str::<Absence>(lesson_json);

    assert!(abs.is_ok(), "{:?}", abs);
    let abs = abs.unwrap();

    assert_eq!(abs.tantargy.nev, "osztályfőnöki");
    assert_eq!(abs.keses_percben, None);
    assert_eq!(abs.rogzito_tanar_neve, "Teszt Lajos");
    assert_eq!(abs.igazolas_allapota, "Igazolt");
    assert!(abs.igazolt());
}

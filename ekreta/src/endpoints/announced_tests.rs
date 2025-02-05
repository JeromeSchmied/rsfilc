use crate::types::{Rektip, Tantargy, Uid};
use crate::{Endpoint, LDateTime, OptIrval, Res};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnnouncedTest {
    pub uid: String,
    pub datum: LDateTime,
    pub bejelentes_datuma: LDateTime,
    pub rogzito_tanar_neve: String,
    pub orarendi_ora_oraszama: Option<u8>,
    pub tantargy: Tantargy,
    pub tantargy_neve: String,
    pub temaja: Option<String>,
    pub modja: Rektip,
    pub osztaly_csoport: Uid,
}

impl Endpoint for AnnouncedTest {
    type Args = OptIrval;

    fn path(_args: &Self::Args) -> String {
        "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek".into()
    }

    fn query(input: &Self::Args) -> Res<impl Serialize> {
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
    let lesson_json = r#"{ "Uid": "00000", "Datum": "2023-09-10T22:00:00Z", "BejelentesDatuma": "2023-09-08T13:35:07Z", "RogzitoTanarNeve": "Teszt Mónika", "OrarendiOraOraszama": 6, "Tantargy": { "Uid": "000000", "Nev": "matematika", "Kategoria": { "Uid": "0000,matematika", "Nev": "matematika", "Leiras": "Matematika" }, "SortIndex": 0 }, "TantargyNeve": "matematika", "Temaja": "Matematikai logika", "Modja": { "Uid": "0000,irasbeli_ropdolgozat", "Nev": "irasbeli_ropdolgozat", "Leiras": "Írásbeli röpdolgozat" }, "OsztalyCsoport": { "Uid": "000000" } }"#;

    let anc = serde_json::from_str::<AnnouncedTest>(lesson_json);

    assert!(anc.is_ok(), "{anc:?}");
    let abs = anc.unwrap();

    assert_eq!(abs.rogzito_tanar_neve, "Teszt Mónika");
    assert_eq!(abs.orarendi_ora_oraszama, Some(6));
    assert_eq!(abs.tantargy_neve, "matematika");
    assert_eq!(abs.temaja, Some("Matematikai logika".into()));
    assert_eq!(abs.modja.leiras, "Írásbeli röpdolgozat");
}

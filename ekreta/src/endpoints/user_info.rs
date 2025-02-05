use crate::{types::*, LDateTime};
use serde::{Deserialize, Serialize};

use super::Endpoint;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub uid: String,
    pub idp_unique_id: String,
    pub tanev_uid: String,
    pub intezmeny_nev: String,
    pub intezmeny_azonosito: String,
    pub nev: String,
    pub szuletesi_nev: String,
    pub szuletesi_hely: String,
    pub anyja_neve: String,
    pub telefonszam: Option<String>,
    pub email_cim: String,
    pub cimek: Vec<String>,
    pub szuletesi_datum: LDateTime,
    pub szuletesi_ev: u16,
    pub szuletesi_honap: u8,
    pub szuletesi_nap: u8,
    pub gondviselok: Vec<Gondviselo>,
    pub bankszamla: Bankszamla,
    pub intezmeny: Intezmeny,
}

impl Endpoint for UserInfo {
    type Args = ();

    fn path(_args: &Self::Args) -> String {
        "/ellenorzo/V3/Sajat/TanuloAdatlap".into()
    }
}

#[cfg(test)]
#[test]
fn parse() {
    let data = r#"{ "Uid": "xxxxxx", "IdpUniqueId": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "TanevUid": "xxxx", "IntezmenyNev": "intezmény neve", "IntezmenyAzonosito": "klikxxxxxxxxx", "Nev": "Diák Név", "SzuletesiNev": "Ugyanaz", "SzuletesiHely": "Makó", "AnyjaNeve": "Egy átlagos Név", "Telefonszam": "+36xxxxxxxxx", "EmailCim": "username@example.com", "Cimek": [ "Makó (xxxx), Petőfi Sándor utca xx. x x" ], "SzuletesiDatum": "2000-01-01T00:00:01Z", "SzuletesiEv": 2000, "SzuletesiHonap": 1, "SzuletesiNap": 1, "Gondviselok": [ { "Uid": "xxxxxx", "IdpUniqueId": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "Nev": "Magyar Név", "EmailCim": "username@mail.com", "Telefonszam": "+36xxxxxxxxx", "IsTorvenyesKepviselo": true, "IsNincsFelugyeletiJoga": false }, { "Uid": "xxxxxx", "IdpUniqueId": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "Nev": "Másik Név", "EmailCim": "usernm@mail.com", "Telefonszam": "+36xxxxxxxxx", "IsTorvenyesKepviselo": true, "IsNincsFelugyeletiJoga": false } ], "Bankszamla": { "BankszamlaSzam": "", "BankszamlaTulajdonosTipusId": null, "BankszamlaTulajdonosNeve": null, "IsReadOnly": false }, "Intezmeny": { "Uid": "xxxx", "TeljesNev": "Iskola", "RovidNev": "Izsgola", "Rendszermodulok": [ { "IsAktiv": true, "Tipus": "Eugyintezes", "Url": null }, { "IsAktiv": false, "Tipus": "LEP", "Url": null }, { "IsAktiv": false, "Tipus": "FeltarGondviselo", "Url": null }, { "IsAktiv": false, "Tipus": "FeltarAszf", "Url": "https://tudasbazis.ekreta.hu/download/attachments/pédéeff.pdf" }, { "IsAktiv": true, "Tipus": "EszkozIgenylesModul", "Url": null }, { "IsAktiv": true, "Tipus": "IsJarmuvezetoKepzesEnable", "Url": null }, { "IsAktiv": true, "Tipus": "NEP", "Url": null } ], "TestreszabasBeallitasok": { "IsDiakRogzithetHaziFeladatot": false, "IsTanorakTemajaMegtekinthetoEllenorzoben": true, "IsOsztalyAtlagMegjeleniteseEllenorzoben": true, "IsElerhetosegSzerkesztheto": true, "ErtekelesekMegjelenitesenekKesleltetesenekMerteke": 6, "KovetkezoTelepitesDatuma": "2020-01-01T20:00:00Z" } } }"#;
    serde_json::from_str::<UserInfo>(&data).unwrap();
}

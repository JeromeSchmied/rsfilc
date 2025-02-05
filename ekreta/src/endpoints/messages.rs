use super::Endpoint;
use crate::{
    types::{Rektip, Uid},
    LDateTime, OptIrval, Res,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Message/Mailbox Item
pub struct MsgItem {
    pub azonosito: u32,
    pub is_elolvasva: bool,
    pub is_torolt_elem: bool,
    pub tipus: Tipus,
    pub uzenet: Msg,
}
impl Endpoint for MsgItem {
    type Args = Option<u32>;

    fn base_url(_args: impl AsRef<str>) -> String {
        super::base::ADMIN.into()
    }

    fn path(id: &Self::Args) -> String {
        format!(
            "/api/v1/kommunikacio/postaladaelemek/{}",
            if let Some(id) = id {
                id.to_string()
            } else {
                String::from("sajat")
            }
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tipus {
    pub azonosito: u32,
    pub kod: String,
    pub rovid_nev: String,
    pub nev: String,
    pub leiras: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Message Overview
pub struct MsgOview {
    pub azonosito: u32,
    pub uzenet_azonosito: u32,
    pub uzenet_kuldes_datum: chrono::NaiveDateTime,
    pub uzenet_felado_nev: Option<String>,
    pub uzenet_felado_titulus: Option<String>,
    pub uzenet_targy: String,
    pub has_csatolmany: bool,
    pub is_elolvasva: bool,
}
impl Endpoint for MsgOview {
    type Args = MsgKind;

    fn base_url(_args: impl AsRef<str>) -> String {
        super::base::ADMIN.into()
    }

    fn path(id: &Self::Args) -> String {
        format!("/api/v1/kommunikacio/postaladaelemek/{}", id.val())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// some kind of additional/system Messag
pub struct NoteMsg {
    pub uid: String,
    pub cim: String,
    pub datum: LDateTime,
    pub keszito_tanar_neve: String,
    pub keszites_datuma: LDateTime,
    pub lattamozas_datuma: Option<LDateTime>,
    pub osztaly_csoport: Option<Uid>,
    pub tartalom: String,
    pub tartalom_formazott: String,
    pub tipus: Rektip,
}
impl Endpoint for NoteMsg {
    type Args = OptIrval;

    fn path(_: &Self::Args) -> String {
        "/ellenorzo/v3/sajat/Feljegyzesek".into()
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Message
pub struct Msg {
    pub azonosito: u32,
    pub kuldes_datum: chrono::NaiveDateTime,
    pub felado_nev: String,
    // #[serde(default)]
    pub felado_titulus: String,
    pub szoveg: String,
    pub targy: String,
    pub cimzett_lista: Vec<Cimzett>,
    pub csatolmanyok: Vec<Attachment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cimzett {
    pub azonosito: u32,
    pub kreta_azonosito: i64,
    pub nev: String,
    pub tipus: Tipus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub azonosito: u32,
    pub fajl_nev: String,
}
impl Endpoint for Attachment {
    type Args = u32;

    fn base_url(_args: impl AsRef<str>) -> String {
        super::base::ADMIN.into()
    }

    fn path(msg_kind: &Self::Args) -> String {
        format!("/api/v1/dokumentumok/uzenetek/{}", msg_kind)
    }
}

/// kinds of [`Msg`]
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize, Eq, PartialOrd, Ord)]
pub enum MsgKind {
    /// recieved
    Recv,
    /// sent
    Sent,
    /// deleted/trashed
    Del,
}

impl From<&String> for MsgKind {
    fn from(value: &String) -> Self {
        match value.to_lowercase().trim_matches('"') {
            "beerkezett" => Self::Recv,
            "elkuldott" => Self::Sent,
            "torolt" => Self::Del,
            v => unreachable!("{v} would be invalid, `Kréta` doesn't do that"),
        }
    }
}
impl MsgKind {
    /// get value for this [`MsgKind`]
    pub fn val(&self) -> String {
        match self {
            MsgKind::Recv => "beerkezett".to_owned(),
            MsgKind::Sent => "elkuldott".to_owned(),
            MsgKind::Del => "torolt".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_overview_parsing() {
        let message_json = r#"{ "azonosito": 137283859, "uzenetAzonosito": 26669244, "uzenetKuldesDatum": "2022-09-07T08:18:17", "uzenetFeladoNev": "Schultz Zoltán", "uzenetFeladoTitulus": "intézményvezető", "uzenetTargy": "Tájékoztató - Elf Bar - Rendőrség", "hasCsatolmany": true, "isElolvasva": true }"#;
        let message = serde_json::from_str::<MsgOview>(message_json);
        let message = message.unwrap();
        assert_eq!(message.azonosito, 137283859);
    }
}

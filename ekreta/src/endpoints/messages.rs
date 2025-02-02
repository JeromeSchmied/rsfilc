use super::Endpoint;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageItem {
    pub azonosito: u32,
    pub is_elolvasva: bool,
    pub is_torolt_elem: bool,
    pub tipus: Tipus,
    pub uzenet: Message,
}
impl Endpoint for MessageItem {
    type QueryInput = ();

    fn base_url(_args: impl AsRef<str>) -> crate::Str {
        super::base::ADMIN.into()
    }

    fn path(id: impl AsRef<str>) -> String {
        let id = id.as_ref();
        format!(
            "/api/v1/kommunikacio/postaladaelemek/{}",
            if id.is_empty() { "sajat" } else { id }
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
pub struct MessageOverview {
    pub azonosito: u32,
    pub uzenet_azonosito: u32,
    pub uzenet_kuldes_datum: chrono::NaiveDateTime,
    pub uzenet_felado_nev: Option<String>,
    pub uzenet_felado_titulus: Option<String>,
    pub uzenet_targy: String,
    pub has_csatolmany: bool,
    pub is_elolvasva: bool,
}
impl Endpoint for MessageOverview {
    type QueryInput = ();

    fn base_url(_args: impl AsRef<str>) -> crate::Str {
        super::base::ADMIN.into()
    }

    fn path(id: impl AsRef<str>) -> String {
        format!("/api/v1/kommunikacio/postaladaelemek/{}", id.as_ref())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
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
    type QueryInput = ();

    fn base_url(_args: impl AsRef<str>) -> crate::Str {
        super::base::ADMIN.into()
    }

    fn path(id: impl AsRef<str>) -> String {
        format!("/api/v1/dokumentumok/uzenetek/{}", id.as_ref())
    }
}

/// kinds of [`Msg`]
#[derive(Debug, PartialEq)]
pub enum MessageKind {
    /// recieved
    Recv,
    /// sent
    Sent,
    /// deleted/trashed
    Del,
}

impl From<&String> for MessageKind {
    fn from(value: &String) -> Self {
        match value.to_lowercase().trim_matches('"') {
            "beerkezett" => Self::Recv,
            "elkuldott" => Self::Sent,
            "torolt" => Self::Del,
            v => unreachable!("{v} would be invalid, `Kréta` doesn't do that"),
        }
    }
}
impl MessageKind {
    /// get value for this [`MsgKind`]
    pub fn val(&self) -> String {
        match self {
            MessageKind::Recv => "beerkezett".to_owned(),
            MessageKind::Sent => "elkuldott".to_owned(),
            MessageKind::Del => "torolt".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_overview_parsing() {
        let message_json = r#"{ "azonosito": 137283859, "uzenetAzonosito": 26669244, "uzenetKuldesDatum": "2022-09-07T08:18:17", "uzenetFeladoNev": "Schultz Zoltán", "uzenetFeladoTitulus": "intézményvezető", "uzenetTargy": "Tájékoztató - Elf Bar - Rendőrség", "hasCsatolmany": true, "isElolvasva": true }"#;
        let message = serde_json::from_str::<MessageOverview>(message_json);
        let message = message.unwrap();
        assert_eq!(message.azonosito, 137283859);
    }
}

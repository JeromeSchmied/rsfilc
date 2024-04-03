//! messaging with teachers and staff

use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// this is just a short representation of the real message
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePreview {
    /// id
    azonosito: u64,
    /// another id
    uzenet_azonosito: u64,

    /// date of sending
    uzenet_kuldes_datum: String,
    /// sender
    uzenet_felado_nev: String,
    /// title
    uzenet_felado_titulus: String,
    /// subject
    uzenet_targy: String,

    /// has attachment
    has_csatolmany: bool,
    /// is read
    is_elolvasva: bool,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
impl MessagePreview {}

/// the message itself
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// id
    azonosito: u32,

    /// is read
    is_elolvasva: bool,
    /// is deleted
    is_torolt_elem: bool,

    /// kind
    tipus: HashMap<String, Value>,

    /// the message itself
    uzenet: HashMap<String, Value>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
impl Message {
    /// Returns the date and time when this [`Message`] was sent.
    ///
    /// # Panics
    ///
    /// Panics if
    /// - message doesn't contain `uzenet` key.
    /// - which doesn't contain `kuldesDatum` key.
    /// - which contains invalid date-time value.
    pub fn time_sent(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(
            &self
                .uzenet
                .get("uzenet")
                .expect("couldn't find info about message")
                .get("kuldesDatum")
                .expect("couldn't find date of sending")
                .to_string(),
        )
        .expect("invalid date-time of date of sending")
        .into()
    }
}

/// kinds of message
pub enum MessageKind {
    Beerkezett,
    Elkuldott,
    Torolt,
}
impl MessageKind {
    pub fn val(&self) -> String {
        match self {
            MessageKind::Beerkezett => "beerkezett".to_owned(),
            MessageKind::Elkuldott => "elkuldott".to_owned(),
            MessageKind::Torolt => "torolt".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_preview_parsing() {
        let message_json = r#"{
        "azonosito": 137283859,
        "uzenetAzonosito": 26669244,
        "uzenetKuldesDatum": "2022-09-07T08:18:17",
        "uzenetFeladoNev": "Schultz Zoltán",
        "uzenetFeladoTitulus": "intézményvezető",
        "uzenetTargy": "Tájékoztató - Elf Bar - Rendőrség",
        "hasCsatolmany": true,
        "isElolvasva": true
    }"#;

        let message = serde_json::from_str::<MessagePreview>(message_json);
        if let Err(e) = &message {
            eprintln!("woohoo: {}", e);
        }
        assert!(message.is_ok());

        let message = message.unwrap();

        assert_eq!(message.azonosito, 137283859);
        assert_eq!(message.uzenet_azonosito, 26669244);

        assert_eq!(message.uzenet_kuldes_datum, "2022-09-07T08:18:17");
        assert_eq!(message.uzenet_felado_nev, "Schultz Zoltán");
        assert_eq!(message.uzenet_felado_titulus, "intézményvezető");
        assert_eq!(message.uzenet_targy, "Tájékoztató - Elf Bar - Rendőrség");

        assert!(message.has_csatolmany);
        assert!(message.is_elolvasva);
    }

    #[test]
    fn message_parsing() {
        let message_json = r#"{
	"azonosito": 1000000,
	"isElolvasva":true,
	"isToroltElem":false,
	"tipus": {
		"azonosito":1,
		"kod":"BEERKEZETT",
		"rovidNev":"Beérkezett üzenet",
		"nev":"Beérkezett üzenet",
		"leiras":"Beérkezett üzenet"
	},
	"uzenet": {
		"azonosito":1000000,
		"kuldesDatum": "1970-01-01T00:00:00",
		"feladoNev":"Dudás Attila",
		"feladoTitulus":"igazgató h.",
		"szoveg":"...",
		"targy":" Tájékoztató ",
		"statusz": {
			"azonosito":2,
			"kod":"KIKULDVE",
			"rovidNev": "Kiküldve",
			"nev":"Kiküldve",
			"leiras":"Kiküldve"
		},
		"cimzettLista": 
		[
			{
				"azonosito": 1000000,
				"kretaAzonosito": 10000,
				"nev":"9.A",
				"tipus": {
					"azonosito":4,
					"kod":"OSZTALY_TANULO",
					"rovidNev":"Osztály - Tanuló",
					"nev":"Osztály - Tanuló",
					"leiras":"Osztály - Tanuló"
				}
			},
			{
				"azonosito":1000000,
				"kretaAzonosito": 100000,
				"nev": "Xxxxxxx Xxxxxxx",
				"tipus": {
					"azonosito":9,
					"kod":"TANAR",
					"rovidNev":"Tanár",
					"nev":"Tanár",
					"leiras":"Tanár"
				}
			}
		],
		"csatolmanyok": [
			{
	                    "azonosito": 1000000,
	                    "fajlNev": "xxxxxxx.xxx"
	                }
		]
	}
}"#;

        let message = serde_json::from_str::<Message>(message_json);
        if let Err(e) = &message {
            eprintln!("woohoo: {}", e);
        }
        assert!(message.is_ok());

        let message = message.unwrap();
    }
}

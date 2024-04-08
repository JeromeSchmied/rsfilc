//! messaging with teachers and staff

use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// this is just a short representation of the real message
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MsgOverview {
    /// id
    pub azonosito: u64,
    /// another id
    // uzenet_azonosito: u64,

    /// date of sending
    uzenet_kuldes_datum: String,
    // /// sender
    // uzenet_felado_nev: String,
    // /// title
    // uzenet_felado_titulus: String,
    // /// subject
    // uzenet_targy: String,

    // /// has attachment
    // has_csatolmany: bool,
    // /// is read
    // is_elolvasva: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
impl MsgOverview {
    /// Returns the date when this [`MessageOverview`] was sent.
    ///
    /// # Panics
    ///
    /// Panics if `uzenet_kuldes_datum` is invalid as date.
    pub fn sent(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&format!("{}Z", &self.uzenet_kuldes_datum))
            .expect("couldn't parse kezdet_idopont")
            .into()
    }
}
// impl fmt::Display for MsgOverview {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "{}", self.sent().format("%Y/%m/%d %H:%M"))?;
//         writeln!(f, "{}", self.uzenet_targy)?;
//         // writeln!(f, "{}", self.uzenet_kuldes_datum)?;
//         // if !self.is_elolvasva {
//         //     writeln!(f, "Olvasatlan")?;
//         // }
//         Ok(())
//     }
// }

/// the message itself
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Msg {
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
impl Msg {
    /// Returns the date and time when this [`Message`] was sent.
    ///
    /// # Panics
    ///
    /// Panics if
    /// - message doesn't contain `kuldesDatum` key.
    /// - which contains invalid date-time value.
    pub fn time_sent(&self) -> DateTime<Local> {
        DateTime::parse_from_rfc3339(&format!("{}Z", self.msg_kv("kuldesDatum")))
            .expect("invalid date-time of date of sending")
            .into()
    }

    fn msg_kv(&self, v: &str) -> String {
        self.uzenet
            .get(v)
            .expect("couldn't find key")
            .to_string()
            .trim_matches('"')
            .to_string()
    }

    pub fn subj(&self) -> String {
        self.msg_kv("targy")
        // self.uzenet
        //     .get("targy")
        //     .expect("couldn't find subject")
        //     .to_string()
    }
    pub fn text(&self) -> String {
        self.msg_kv("szoveg")
    }
    pub fn sender(&self) -> String {
        self.msg_kv("feladoNev")
    }
    pub fn sender_title(&self) -> String {
        self.msg_kv("feladoTitulus")
    }
}
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Kiküldve: {}", self.time_sent().format("%Y/%m/%d %H:%M"))?;
        writeln!(f, "Feladó: {} {}", self.sender(), self.sender_title())?;
        writeln!(f, "Tárgy: {}", self.subj())?;
        writeln!(f, "Üzenet: {}", self.text())?;
        writeln!(f)?;
        // writeln!(f, "{}", self.uzenet_kuldes_datum)?;
        // if !self.is_elolvasva {
        //     writeln!(f, "Olvasatlan")?;
        // }
        Ok(())
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
    fn message_overview_parsing() {
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

        let message = serde_json::from_str::<MsgOverview>(message_json);
        if let Err(e) = &message {
            eprintln!("woohoo: {}", e);
        }
        assert!(message.is_ok());

        let message = message.unwrap();

        assert_eq!(message.azonosito, 137283859);
        // assert_eq!(message.uzenet_azonosito, 26669244);

        assert_eq!(message.uzenet_kuldes_datum, "2022-09-07T08:18:17");
        // assert_eq!(message.uzenet_felado_nev, "Schultz Zoltán");
        // assert_eq!(message.uzenet_felado_titulus, "intézményvezető");
        // assert_eq!(message.uzenet_targy, "Tájékoztató - Elf Bar - Rendőrség");

        // assert!(message.has_csatolmany);
        // assert!(message.is_elolvasva);
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

        let message = serde_json::from_str::<Msg>(message_json);
        if let Err(e) = &message {
            eprintln!("woohoo: {}", e);
        }
        assert!(message.is_ok());

        let message = message.unwrap();
        assert_eq!(message.azonosito, 1000000);
    }
}

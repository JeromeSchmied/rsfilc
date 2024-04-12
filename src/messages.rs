//! messaging with teachers and staff

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fmt};

/// this is just a short representation of the real message
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MsgOview {
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
    _extra: HashMap<String, Value>,
}
impl MsgOview {
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
    // /// id
    // azonosito: u32,

    // /// is read
    // is_elolvasva: bool,
    // /// is deleted
    // is_torolt_elem: bool,

    // /// kind
    // tipus: HashMap<String, Value>,
    /// the message itself
    uzenet: HashMap<String, Value>,

    #[serde(flatten)]
    _extra: HashMap<String, Value>,
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

    /// Get `value` for `k`ey.
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `k`ey.
    fn msg_kv(&self, k: &str) -> String {
        self.uzenet
            .get(k)
            .expect("couldn't find key")
            .to_string()
            .trim_matches('"')
            .to_string()
    }

    /// Returns the `subject` of this [`Msg`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `subject`.
    pub fn subj(&self) -> String {
        self.msg_kv("targy")
    }
    /// Returns the `text` of this [`Msg`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `text`.
    pub fn text(&self) -> String {
        self.msg_kv("szoveg")
    }
    /// Returns the `sender` of this [`Msg`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `sender`.
    pub fn sender(&self) -> String {
        self.msg_kv("feladoNev")
    }
    /// Returns the `sender_title` of this [`Msg`].
    ///
    /// # Panics
    ///
    /// Panics if data doesn't contain `sender_title`.
    pub fn sender_title(&self) -> String {
        self.msg_kv("feladoTitulus")
    }

    /// ˝render˝ html to console
    fn render_html(html: &str) -> String {
        let html = html.replace('\\', "");

        let mut text = String::new();
        let mut is_attr = false;
        let mut attr = String::new();

        for ch in html.chars() {
            if ch == '<' {
                is_attr = true;
            } else if ch == '>' {
                is_attr = false;

                if attr.contains('/') {
                    text.push('\n');
                }
                // if !attr.is_empty() {
                //     let attr = attr.trim();
                // }
            }

            if is_attr {
                attr.push(ch);
            } else {
                attr.clear();

                text.push(ch);
            }
        }

        text.replace('>', "").replace("\n\n\n", "\n")
    }
}
impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Kiküldve: {}", self.time_sent().format("%Y/%m/%d %H:%M"))?;
        writeln!(f, "Feladó: {} {}", self.sender(), self.sender_title())?;
        writeln!(f, "Tárgy: {}", self.subj())?;
        writeln!(f, "\n{}", Self::render_html(&self.text()))?;
        writeln!(f, "---------------------------------\n")?;
        // writeln!(f, "{}", self.uzenet_kuldes_datum)?;
        // if !self.is_elolvasva {
        //     writeln!(f, "Olvasatlan")?;
        // }
        Ok(())
    }
}

/// kinds of [`Msg`]
pub enum MsgKind {
    /// recieved
    Recv,
    /// sent
    Sent,
    /// deleted/trashed
    Del,
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

        let message = serde_json::from_str::<MsgOview>(message_json);
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
        let msg_json = r#"{
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

        let msg = serde_json::from_str::<Msg>(msg_json);
        if let Err(e) = &msg {
            eprintln!("woohoo: {}", e);
        }
        assert!(msg.is_ok());

        let msg = msg.unwrap();
        assert_eq!(msg.sender(), "Dudás Attila");
        assert_eq!(msg.sender_title(), "igazgató h.");
    }
}

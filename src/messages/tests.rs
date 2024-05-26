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

    assert_eq!(message.id, 137283859);
    // assert_eq!(message.uzenet_azonosito, 26669244);

    assert_eq!(message.date_sent, "2022-09-07T08:18:17");
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
    assert_eq!(
        msg.attachments(),
        vec![Attachment {
            file_name: "xxxxxxx.xxx".to_string(),
            id: 1000000
        }]
    );
    // assert_eq!(msg.kind(), Some(MsgKind::Recv));
    assert_eq!(msg.sender(), "Dudás Attila");
    assert_eq!(msg.sender_title(), Some("igazgató h.".to_string()));
}
#[test]
fn fix_file_name() {
    let f = Attachment {
        file_name: "ain't a good filename is it ted? .txt .doksi .docx".to_string(),
        id: 38,
    };
    assert_eq!(
        download_dir().join("ain't_a_good_filename_is_it_ted?_.txt_.doksi_.docx"),
        f.download_to()
    );
}

#[test]
fn naughty_msg_deser() {
    let msg_s = r#"{
        "Uid": "862931",
        "Cim": "Tanévkezdés gimnazistáknak",
        "Datum": "2023-09-01T13:45:03Z",
        "KeszitoTanarNeve": "Rendszerüzenet",
        "KeszitesDatuma": "2023-09-01T13:45:03Z",
        "LattamozasDatuma": null,
        "OsztalyCsoport": null,
        "Tartalom": "\r\nKérjük olvassa el Dr. Pintér Sándor belügyminiszter tanévkezdéshez kapcsolódó, tájékoztató levelét; amelyet az alábbi hivatkozáson keresztül érhet el!\r\n\r\nhttps://tudasbazis.ekreta.hu/download/attachments/2424949/BM_KR%C3%89TA_kommunik%C3%A1ci%C3%B3_gimn.pdf?api=v2\r\n",
        "TartalomFormazott": "\r\n<p>Kérjük olvassa el Dr. Pintér Sándor belügyminiszter tanévkezdéshez kapcsolódó, tájékoztató levelét; amelyet <strong><a href=\"https://tudasbazis.ekreta.hu/download/attachments/2424949/BM_KR%C3%89TA_kommunik%C3%A1ci%C3%B3_gimn.pdf?api=v2\" target=\"_blank\">IDE</a></strong> kattintva érhet el!</p>\r\n",
        "Tipus": {
            "Uid": "5482,ElektronikusUzenet",
            "Nev": "ElektronikusUzenet",
            "Leiras": "Elektronikus üzenet"
        }
    }"#;

    let msg = serde_json::from_str::<NoteMsg>(msg_s);
    assert!(msg.is_ok());

    let msg = msg.unwrap();
    assert_eq!(msg.title, "Tanévkezdés gimnazistáknak");
    // assert_eq!(msg.date, "2023-09-01T13:45:03Z");
    assert_eq!(msg.teacher, "Rendszerüzenet");
    assert_eq!(msg.msg, "\r\n<p>Kérjük olvassa el Dr. Pintér Sándor belügyminiszter tanévkezdéshez kapcsolódó, tájékoztató levelét; amelyet <strong><a href=\"https://tudasbazis.ekreta.hu/download/attachments/2424949/BM_KR%C3%89TA_kommunik%C3%A1ci%C3%B3_gimn.pdf?api=v2\" target=\"_blank\">IDE</a></strong> kattintva érhet el!</p>\r\n");
}

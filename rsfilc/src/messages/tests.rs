use super::*;

#[test]
fn fix_file_name() {
    let f = ekreta::Attachment {
        fajl_nev: "ain't a good filename is it ted? .txt .doksi .docx".to_string(),
        azonosito: 38,
    };
    assert_eq!(
        download_dir().join("ain't_a_good_filename_is_it_ted?_.txt_.doksi_.docx"),
        download_attachment_to(&f)
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

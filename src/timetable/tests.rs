use super::*;

#[test]
fn works() {
    let lesson_json = r#"{
        "Uid": "XXXXXXX,TanitasiOra,2024-03-17T23:00:00Z",
        "Datum": "2024-03-17T23:00:00Z",
        "KezdetIdopont": "2024-03-18T08:50:00Z",
        "VegIdopont": "2024-03-18T09:35:00Z",
        "Nev": "fizika",
        "Oraszam": 3,
        "OraEvesSorszama": 72,
        "OsztalyCsoport": {
            "Uid": "837087",
            "Nev": "XX.X"
        },
        "TanarNeve": "Teszt Katalin",
        "Tantargy": {
            "Uid": "368813",
            "Nev": "fizika",
            "Kategoria": {
                "Uid": "1219,fizika",
                "Nev": "fizika",
                "Leiras": "Fizika"
            },
            "SortIndex": 0
        },
        "Tema": "Félvezetők",
        "TeremNeve": "Fizika",
        "Tipus": {
            "Uid": "2,TanitasiOra",
            "Nev": "TanitasiOra",
            "Leiras": "Tanítási óra"
        },
        "TanuloJelenlet": {
            "Uid": "XXXX,Jelenlet",
            "Nev": "Jelenlet",
            "Leiras": "A tanuló részt vett a tanórán"
        },
        "Allapot": {
            "Uid": "1,Naplozott",
            "Nev": "Naplozott",
            "Leiras": "Naplózott"
        },
        "HelyettesTanarNeve": null,
        "HaziFeladatUid": null,
        "FeladatGroupUid": null,
        "NyelviFeladatGroupUid": null,
        "BejelentettSzamonkeresUid": null,
        "IsTanuloHaziFeladatEnabled": false,
        "IsHaziFeladatMegoldva": false,
        "Csatolmanyok": [],
        "IsDigitalisOra": false,
        "DigitalisEszkozTipus": "Na",
        "DigitalisPlatformTipus": "Na",
        "DigitalisTamogatoEszkozTipusList": ["Na"],
        "Letrehozas": "2023-08-26T18:15:00",
        "UtolsoModositas": "2023-08-26T18:15:00"
    }"#;

    let lesson = serde_json::from_str::<Lesson>(lesson_json);

    assert!(lesson.is_ok(), "{:?}", lesson);
    let lesson = lesson.unwrap();

    assert_eq!(lesson.subject, "fizika");
    assert_eq!(lesson.room, Some("Fizika".to_string()));
    assert_eq!(lesson.topic, Some("Félvezetők".to_string()));
    assert_eq!(lesson.start, "2024-03-18T08:50:00Z");
    assert_eq!(lesson.end, "2024-03-18T09:35:00Z");
    assert_eq!(lesson.teacher, Some("Teszt Katalin".to_string()));
    assert_eq!(lesson.alt_teacher, None);
    assert!(!lesson.cancelled());
    assert!(!lesson.absent());
    assert_eq!(lesson.subject_id(), Some("fizika".to_string()));
    assert!(!lesson.shite());
}

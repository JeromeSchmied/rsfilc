use super::*;

#[test]
fn works() {
    let lesson_json = r#"{
        "Uid": "00000",
        "Datum": "2023-09-10T22:00:00Z",
        "BejelentesDatuma": "2023-09-08T13:35:07Z",
        "RogzitoTanarNeve": "Teszt Mónika",
        "OrarendiOraOraszama": 6,
        "Tantargy": {
            "Uid": "000000",
            "Nev": "matematika",
            "Kategoria": {
                "Uid": "0000,matematika",
                "Nev": "matematika",
                "Leiras": "Matematika"
            },
            "SortIndex": 0
        },
        "TantargyNeve": "matematika",
        "Temaja": "Matematikai logika",
        "Modja": {
            "Uid": "0000,irasbeli_ropdolgozat",
            "Nev": "irasbeli_ropdolgozat",
            "Leiras": "Írásbeli röpdolgozat"
        },
        "OsztalyCsoport": {
            "Uid": "000000"
        }
    }"#;

    let anc = serde_json::from_str::<Ancd>(lesson_json);

    assert!(anc.is_ok(), "{:?}", anc);
    let abs = anc.unwrap();

    assert_eq!(abs.teacher_entered, "Teszt Mónika");
    assert_eq!(abs.nth, Some(6));
    assert_eq!(abs.subject, "matematika");
    assert_eq!(abs.topic, Some("Matematikai logika".into()));
    assert_eq!(abs.kind(), "Írásbeli röpdolgozat");
}

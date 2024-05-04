use super::*;

#[test]
fn works() {
    let lesson_json = r#"{
        "Uid": "00000000",
        "Tantargy": {
            "Uid": "000000",
            "Nev": "osztályfőnöki",
            "Kategoria": {
                "Uid": "0000,egyeb",
                "Nev": "egyeb",
                "Leiras": "Egyéb"
            },
            "SortIndex": 0
        },
        "Ora": {
            "KezdoDatum": "2023-09-01T06:55:00Z",
            "VegDatum": "2023-09-01T07:40:00Z",
            "Oraszam": 2
        },
        "Datum": "2023-08-31T22:00:00Z",
        "RogzitoTanarNeve": "Teszt Lajos",
        "Tipus": {
            "Uid": "1500,hianyzas",
            "Nev": "hianyzas",
            "Leiras": "Hiányzás"
        },
        "Mod": {
            "Uid": "1,Tanorai",
            "Nev": "Tanorai",
            "Leiras": "Tanórai mulasztás"
        },
        "KesesPercben": null,
        "KeszitesDatuma": "2023-09-02T08:09:19Z",
        "IgazolasAllapota": "Igazolt",
        "IgazolasTipusa": {
            "Uid": "0000,Kikero",
            "Nev": "Kikero",
            "Leiras": "Kikérő"
        },
        "OsztalyCsoport": {
            "Uid": "000000"
        }
    }"#;

    let abs = serde_json::from_str::<Abs>(lesson_json);

    assert!(abs.is_ok(), "{:?}", abs);
    let abs = abs.unwrap();

    assert_eq!(abs.subj(), "osztályfőnöki");
    assert_eq!(abs.mins_late, None);
    assert_eq!(abs.teacher, "Teszt Lajos");
    assert_eq!(abs.verification_status, "Igazolt");
    assert!(abs.verified());
}

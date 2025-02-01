use super::*;

#[test]
fn works() {
    let eval_json = r#"{
        "Uid": "00000000,Ertekeles",
        "RogzitesDatuma": "2024-01-16T23:00:00Z",
        "KeszitesDatuma": "2024-01-18T16:48:53Z",
        "LattamozasDatuma": null,
        "Tantargy": {
            "Uid": "368800",
            "Nev": "irodalom",
            "Kategoria": {
                "Uid": "1198,magyar_nyelv_es_irodalom",
                "Nev": "magyar_nyelv_es_irodalom",
                "Leiras": "Magyar nyelv és irodalom"
            },
            "SortIndex": 3
        },
        "Tema": "Villon",
        "Tipus": {
            "Uid": "1518,evkozi_jegy_ertekeles",
            "Nev": "evkozi_jegy_ertekeles",
            "Leiras": "Évközi jegy/értékelés"
        },
        "Mod": {
            "Uid": "000000,AdatszotarElem",
            "Nev": "AdatszotarElem",
            "Leiras": "Memoriter"
        },
        "ErtekFajta": {
            "Uid": "1,Osztalyzat",
            "Nev": "Osztalyzat",
            "Leiras": "Elégtelen (1) és Jeles (5) között az öt alapértelmezett érték"
        },
        "ErtekeloTanarNeve": "Teszt Tamás",
        "Jelleg": "Ertekeles",
        "SzamErtek": 5,
        "SzovegesErtek": "Jeles(5)",
        "SulySzazalekErteke": 100,
        "SzovegesErtekelesRovidNev": null,
        "OsztalyCsoport": {
            "Uid": "837087"
        },
        "SortIndex": 3
    }"#;

    let eval = serde_json::from_str::<Eval>(eval_json);
    assert!(eval.is_ok());

    let eval = eval.unwrap();

    assert_eq!(eval.topic, Some("Villon".to_string()));
    assert_eq!(eval.teacher, Some("Teszt Tamás".to_owned()));
    assert_eq!(eval.as_num, Some(5));
    assert_eq!(eval.as_txt, "Jeles(5)");
    assert_eq!(eval.weight_in_percent, Some(100));
    assert_eq!(
        eval.subject_id(),
        Some("magyar_nyelv_es_irodalom".to_owned())
    );
    assert_eq!(
        eval.subject_name(),
        Some("Magyar nyelv és irodalom".to_owned())
    );
    assert_eq!(eval.kind(), Some("Memoriter".to_owned()));
    assert_eq!(eval.multi_from_percent(), 1.);
    assert_eq!(eval.type_id(), Some("evkozi_jegy_ertekeles".to_owned()))
}

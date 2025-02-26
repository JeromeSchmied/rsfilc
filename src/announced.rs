//! Announced tests

use crate::{time::MyDate, user::Usr, utils};
use chrono::{Datelike, Local};
use ekreta::{AnnouncedTest, Res};

pub fn handle(past: bool, user: &Usr, subj: Option<String>, rev: bool, num: usize) -> Res<()> {
    let mut all_announced = user.get_tests((None, None))?;
    if !past {
        let today = Local::now().num_days_from_ce();
        all_announced.retain(|ancd| ancd.datum.num_days_from_ce() >= today);
    }
    if let Some(subject) = subj {
        filter_by_subject(&mut all_announced, &subject);
    }
    let headers = ["TÉMA", "TANTÁRGY", "DÁTUM", "MÓD", "TANÁR"];
    utils::print_table(&all_announced, headers.into_iter(), rev, num, disp);
    Ok(())
}

/// filter [`Ancd`] tests by `subj`ect
pub fn filter_by_subject(ancds: &mut Vec<AnnouncedTest>, subj: &str) {
    log::info!("filtering announced tests by subject: {}", subj);
    ancds.retain(|ancd| {
        ancd.tantargy_neve
            .to_lowercase()
            .contains(&subj.to_lowercase())
    });
}

pub fn disp(ancd: &AnnouncedTest) -> Vec<String> {
    let about = ancd.temaja.clone().unwrap_or_default();
    let subj = ancd.tantargy_neve.clone();
    let date = ancd.datum.pretty();
    let kind = ancd.modja.leiras.clone();
    let teacher = ancd.rogzito_tanar_neve.clone();

    vec![about, subj, date, kind, teacher]
}

//! Announced tests

use crate::{time::MyDate, user::Usr, utils};
use chrono::{Datelike, Local};
use ekreta::{AnnouncedTest, Res};
use std::fmt::Write;

pub fn handle(past: bool, user: &Usr, subj: Option<String>, rev: bool, num: usize) -> Res<()> {
    let mut all_announced = user.get_tests((None, None))?;
    if !past {
        let today = Local::now().num_days_from_ce();
        all_announced.retain(|ancd| ancd.datum.num_days_from_ce() >= today);
    }
    if let Some(subject) = subj {
        filter_by_subject(&mut all_announced, &subject);
    }
    utils::print_to_or_rev(&all_announced, num, rev, disp);
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

pub fn disp(ancd: &AnnouncedTest) -> String {
    let mut f = String::new();
    _ = write!(&mut f, "| {}", &ancd.datum.pretty());
    _ = write!(&mut f, " {}", ancd.tantargy_neve);
    if let Some(tc) = &ancd.temaja {
        _ = write!(&mut f, ": {tc}");
    }

    _ = writeln!(&mut f, "\n| {}", ancd.modja.leiras);
    _ = writeln!(&mut f, "| {}", ancd.rogzito_tanar_neve);
    _ = write!(
        &mut f,
        "| Rögzítés dátuma: {}",
        &ancd.bejelentes_datuma.pretty()
    );
    f
}

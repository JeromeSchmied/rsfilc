//! Announced tests

use crate::MyDate;
use ekreta::AnnouncedTest;
use log::info;
use std::fmt::Write;

/// filter [`Ancd`] tests by `subj`ect
pub fn filter_by_subject(ancds: &mut Vec<AnnouncedTest>, subj: &str) {
    info!("filtering announced tests by subject: {}", subj);
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
        _ = write!(&mut f, ": {}", tc);
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

//! Announced tests

use crate::{fill, time::MyDate, user::Usr};
use chrono::Local;
use ekreta::{AnnouncedTest, Res};
use std::fmt::Write;

pub fn handle(past: bool, user: &Usr, subj: Option<String>, rev: bool, num: usize) -> Res<()> {
    let from = if past { None } else { Some(Local::now()) };
    let mut all_announced = user.get_all_announced((from, None))?;
    if let Some(subject) = subj {
        filter_by_subject(&mut all_announced, &subject);
    }
    if rev {
        for announced in all_announced.iter().take(num).rev() {
            let as_str = disp(announced);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    } else {
        for announced in all_announced.iter().take(num) {
            let as_str = disp(announced);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    }
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

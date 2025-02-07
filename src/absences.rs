//! Absences

use crate::{fill, time::MyDate, user::Usr};
use ekreta::{Absence, Res};
use std::fmt::Write;

pub fn handle(user: &Usr, subj: Option<String>, count: bool, rev: bool, num: usize) -> Res<()> {
    let mut absences = user.get_absences((None, None))?;
    if let Some(subject) = subj {
        filter_by_subject(&mut absences, &subject);
    }
    if count {
        println!("Összes hiányzásod száma: {}", absences.len());
        println!(
            "Ebből még igazolatlan: {}",
            absences.iter().filter(|item| !item.igazolt()).count()
        );
        return Ok(());
    }
    if rev {
        for absence in absences.iter().take(num).rev() {
            let as_str = disp(absence);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    } else {
        for absence in absences.iter().take(num) {
            let as_str = disp(absence);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    }
    Ok(())
}

/// filter [`Abs`]ences by `subj`ect
pub fn filter_by_subject(abss: &mut Vec<Absence>, subj: &str) {
    log::info!("filtering absences by subject: {}", subj);
    abss.retain(|abs| {
        abs.tantargy
            .nev
            .to_lowercase()
            .contains(&subj.to_lowercase())
    });
}

pub fn disp(abs: &Absence) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", abs.tantargy.nev);
    _ = writeln!(&mut f, "| {}", abs.rogzito_tanar_neve);
    _ = writeln!(
        f,
        "| {} -> {}",
        &abs.ora.kezdo_datum.pretty(),
        &abs.ora.veg_datum.pretty(),
    );
    if let Some(late) = &abs.keses_percben {
        _ = writeln!(&mut f, "| Késtél {late} percet");
    }

    if abs.igazolt() {
        _ = write!(&mut f, "| igazolt");
    } else {
        _ = write!(&mut f, "| igazolatlan");
    }
    f
}

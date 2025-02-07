//! Absences

use crate::time::MyDate;
use ekreta::Absence;
use std::fmt::Write;

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

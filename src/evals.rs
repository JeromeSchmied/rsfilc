//! evaluations/grades the user received

use crate::{time::MyDate, user::Usr, utils};
use ekreta::{Evaluation, Res};
use log::info;
use std::fmt::Write;

pub fn handle(
    user: &Usr,
    filter: Option<String>,
    subj: Option<String>,
    ghost: &[u8],
    avg: bool,
    rev: bool,
    num: usize,
) -> Res<()> {
    let mut evals = user.get_evals((None, None))?;
    info!("got evals");
    if let Some(kind) = filter {
        filter_by_kind_or_title(&mut evals, &kind);
    }
    if let Some(subject) = subj {
        filter_by_subject(&mut evals, &subject);
    }
    if avg {
        let avg = calc_average(&evals, ghost);
        println!("Average: {avg:.2}");

        return Ok(());
    }
    utils::print_to_or_rev(&evals, num, rev, disp);
    Ok(())
}

/// Filter `evals` by `kind`
pub fn filter_by_kind_or_title(evals: &mut Vec<Evaluation>, filter: &str) {
    let filter = filter.to_lowercase();
    log::info!("filtering evals by kind: {}", filter);
    evals.retain(|eval| {
        eval.r#mod
            .as_ref()
            .is_some_and(|m| m.leiras.to_lowercase().contains(&filter))
            || eval
                .tema
                .as_ref()
                .is_some_and(|t| t.to_lowercase().contains(&filter))
            || eval.tipus.leiras.to_lowercase().contains(&filter)
    });
}

/// Filter `evals` by `subject`
pub fn filter_by_subject(evals: &mut Vec<Evaluation>, subj: &str) {
    log::info!("filtering evals by subject: {}", subj);
    evals.retain(|eval| {
        eval.tantargy
            .nev
            .to_lowercase()
            .contains(&subj.to_lowercase())
    });
}

/// Calculate average of `evals` and `ghosts` evals
pub fn calc_average(evals: &[Evaluation], ghosts: &[u8]) -> f32 {
    log::info!("calculating average for evals");
    let evals = evals
        .iter()
        .filter(|eval| !eval.evvegi() && !eval.felevi() && eval.szam_ertek.is_some());

    // filter it, so only valid grades retain
    let ghosts = ghosts.iter().filter(|g| *g > &0 && *g <= &5);

    let sum = evals.clone().fold(0., |sum, cur| {
        sum + cur.szam_ertek.unwrap_or(0) as f32 * cur.szorzo()
    }) + ghosts.clone().fold(0., |sum, num| sum + *num as f32);

    let count = evals.clone().fold(0., |sum, cur| sum + cur.szorzo()) + ghosts.count() as f32;

    sum / count
}

pub fn disp(eval: &Evaluation) -> String {
    let mut f = String::new();
    _ = write!(&mut f, "| ");
    if let Some(desc) = &eval.tema {
        _ = write!(&mut f, "{desc}: ");
    }
    _ = writeln!(&mut f, "{}", eval.szoveges_ertek);
    _ = writeln!(&mut f, "| {}", eval.tantargy.nev);
    if let Some(m) = &eval.r#mod {
        _ = writeln!(&mut f, "| {}", m.leiras);
    }

    _ = writeln!(&mut f, "| {}", eval.tipus.leiras);
    if let Some(teacher) = &eval.ertekelo_tanar_neve {
        _ = writeln!(&mut f, "| {teacher}");
    }
    _ = write!(&mut f, "| Időpont: {}", &eval.keszites_datuma.pretty());

    f
}

//! evaluations/grades the user received

use crate::{time::MyDate, user::Usr, utils};
use ascii_table::AsciiTable;
use ekreta::{Evaluation, Res};
use log::info;

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

    let mut table = AsciiTable::default();
    if let Some(size) = termsize::get() {
        table.set_max_width(size.cols.into());
    }
    table.column(0).set_header("TÉMA");
    table.column(1).set_header("JEGY");
    table.column(2).set_header("TANTÁRGY");
    table.column(3).set_header("MÓD");
    table.column(4).set_header("TÍPUS");
    table.column(5).set_header("TANÁR");
    table.column(6).set_header("IDŐPONT");
    let data = evals.into_iter().map(disp).collect::<Vec<_>>();
    table.print(data);

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

pub fn disp(eval: Evaluation) -> Vec<String> {
    let desc = eval.tema.clone().unwrap_or_default();
    let grade = if let Some(num) = eval.szam_ertek {
        num.to_string()
    } else {
        eval.szoveges_ertek.clone()
    };
    let subj_name = eval.tantargy.nev.clone();
    let how_desc = eval.r#mod.clone().map(|l| l.leiras).unwrap_or_default();
    let kind_name = eval.tipus.leiras.replace(" jegy/értékelés", "");
    let teacher = eval.ertekelo_tanar_neve.clone().unwrap_or_default();
    let when = eval.keszites_datuma.pretty();

    vec![
        desc, grade, subj_name, how_desc, /* kind_name, */ teacher, when,
    ]
}

//! messages from teachers and staff

use crate::{paths::download_dir, time::MyDate, user::Usr, utils};
use ekreta::{Endpoint, Res};
use std::{char, fmt::Write};

pub fn handle_note_msgs(user: &Usr, id: Option<usize>, args: &crate::Args) -> Res<()> {
    let notes = user.get_note_msgs((None, None))?;
    if let Some(ix) = id {
        let Some(nm) = notes.get(ix) else {
            return Err(format!("can't find message with id: {ix}").into());
        };
        let print = disp_nm(nm);
        println!("{print}");
        return Ok(());
    }

    let data = notes.iter().enumerate().collect::<Vec<_>>();
    let headers = ["id", "tárgya", "tőle", "ekkor"].iter();
    #[rustfmt::skip]
    let disp = if args.machine { None } else { Some(preview_nm) };
    utils::print_table(&data, headers, args.reverse, args.number, disp)
}

pub fn handle(user: &Usr, id: Option<usize>, args: &crate::Args) -> Res<()> {
    let msg_oviews = user.fetch_msg_oviews()?;
    if let Some(ix) = id {
        let msg_oview = msg_oviews
            .get(ix)
            .ok_or(format!("can't find message with id: {ix}"))?;
        let msg = user.get_msg(msg_oview)?;
        let print = disp_msg(&msg);
        println!("{print}");
        return Ok(());
    }

    let data = msg_oviews.iter().enumerate().collect::<Vec<_>>();
    let headers = ["id", "tárgya", "tőle", "ekkor", "csatolmánya"].iter();
    #[rustfmt::skip]
    let disp = if args.machine { None } else { Some(disp_oviews) };
    utils::print_table(&data, headers, args.reverse, args.number, disp)
}

fn disp_oviews(preview: &(usize, &ekreta::MsgOview)) -> Vec<String> {
    let msg = preview.1;
    let datetime = msg.when().unwrap().pretty();
    let subj = msg.uzenet_targy.clone();
    let prefix = msg.uzenet_felado_nev.clone().unwrap_or_default();
    let name = msg.uzenet_felado_titulus.clone().unwrap_or_default();
    let sender = format!("{prefix} {name}");
    // if !msg_oview.is_elolvasva {
    //     writeln!(f, "Olvasatlan")?;
    // }
    let n = preview.0.to_string();
    let mut row = vec![n, subj, sender, datetime];
    if msg.has_csatolmany {
        row.push(String::from("van"));
    }
    row
}

pub fn download_attachment_to(am: &ekreta::Attachment) -> std::path::PathBuf {
    download_dir().join(am.fajl_nev.replace(char::is_whitespace, "_"))
}

pub fn disp_msg(msg: &ekreta::MsgItem) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| Tárgy: {}", msg.uzenet.targy);
    for am in &msg.uzenet.csatolmanyok {
        let out_path = download_attachment_to(am);
        _ = writeln!(&mut f, "| Csatolmány: \"file://{}\"", out_path.display());
    }
    let name = &msg.tipus.nev;
    _ = writeln!(&mut f, "| {name}: {}", msg.when().unwrap().pretty());
    let sender = &msg.uzenet.felado_nev;
    _ = writeln!(&mut f, "| Feladó: {sender} {}", msg.uzenet.felado_titulus);
    let rendered = nanohtml2text::html2text(&msg.uzenet.szoveg);
    _ = write!(&mut f, "\n{rendered}");
    // if !msg.is_elolvasva {
    //  writeln!(f, "Olvasatlan")?;
    // }
    f
}

pub fn preview_nm(preview: &(usize, &ekreta::NoteMsg)) -> Vec<String> {
    let note_msg = preview.1;
    let n = preview.0.to_string();
    let subj = note_msg.cim.clone();
    let datetime = note_msg.datum.pretty();
    let sender = note_msg.keszito_tanar_neve.clone();

    vec![n, subj, sender, datetime]
}

pub fn disp_nm(note_msg: &ekreta::NoteMsg) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", note_msg.cim);
    _ = writeln!(&mut f, "| Időpont: {}", note_msg.datum.pretty());
    _ = writeln!(&mut f, "| {}", note_msg.keszito_tanar_neve);
    let rendered = nanohtml2text::html2text(&note_msg.tartalom_formazott);
    _ = write!(&mut f, "\n{rendered}",);
    f
}

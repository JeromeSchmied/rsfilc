//! messages from teachers and staff

use crate::{paths::download_dir, time::MyDate, user::Usr, utils};
use ekreta::Res;
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
    let msgs = user.msgs((None, None))?;
    if let Some(ix) = id {
        let Some(msg) = msgs.get(ix) else {
            return Err(format!("can't find message with id: {ix}").into());
        };
        let print = disp_msg(msg);
        println!("{print}");
        return Ok(());
    }

    let data = msgs.iter().enumerate().collect::<Vec<_>>();
    let headers = ["id", "tárgya", "tőle", "ekkor", "ilyen", "csatolmány db"].iter();
    #[rustfmt::skip]
    let disp = if args.machine { None } else { Some(preview_msg) };
    utils::print_table(&data, headers, args.reverse, args.number, disp)
}

// impl fmt::Display for MsgOverview {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "{}", self.sent().format("%Y/%m/%d %H:%M"))?;
//         writeln!(f, "{}", self.uzenet_targy)?;
//         // writeln!(f, "{}", self.uzenet_kuldes_datum)?;
//         // if !self.is_elolvasva {
//         //     writeln!(f, "Olvasatlan")?;
//         // }
//         Ok(())
//     }
// }

pub fn download_attachment_to(am: &ekreta::Attachment) -> std::path::PathBuf {
    download_dir().join(am.fajl_nev.replace(char::is_whitespace, "_"))
}

fn preview_msg(preview: &(usize, &ekreta::MsgItem)) -> Vec<String> {
    let msg = preview.1;
    let subj = msg.uzenet.targy.clone();
    let attachment_count = msg.uzenet.csatolmanyok.len().to_string();

    let kind = msg.tipus.nev.replace(" üzenet", "");
    let datetime = msg
        .uzenet
        .kuldes_datum
        .and_local_timezone(chrono::Local)
        .unwrap()
        .pretty();
    let sender = format!("{} {}", msg.uzenet.felado_nev, msg.uzenet.felado_titulus);
    let n = preview.0.to_string();
    vec![n, subj, sender, kind, datetime, attachment_count]
}

pub fn disp_msg(msg: &ekreta::MsgItem) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| Tárgy: {}", msg.uzenet.targy);
    for am in &msg.uzenet.csatolmanyok {
        _ = writeln!(
            &mut f,
            "| Csatolmány: \"file://{}\"",
            download_attachment_to(am).display()
        );
    }

    _ = writeln!(
        &mut f,
        "| {}: {}",
        msg.tipus.nev,
        &msg.uzenet
            .kuldes_datum
            .and_local_timezone(chrono::Local)
            .unwrap()
            .pretty()
    );
    _ = writeln!(
        &mut f,
        "| Feladó: {} {}",
        msg.uzenet.felado_nev, msg.uzenet.felado_titulus
    );
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

//! messages from teachers and staff

use crate::{paths::download_dir, time::MyDate, user::Usr, utils};
use ekreta::Res;
use std::{char, fmt::Write};

pub fn handle(notes: bool, user: &Usr, args: crate::Args) -> Res<()> {
    if notes {
        let notes = user.get_note_msgs((None, None))?;
        let disp = if args.machine { None } else { Some(disp_nm) };
        return utils::print_to_or_rev(&notes, args.number, args.reverse, disp);
    }
    let msgs = user.msgs((None, None))?;
    let disp = if args.machine { None } else { Some(disp_msg) };
    utils::print_to_or_rev(&msgs, args.number, args.reverse, disp)
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

pub fn disp_nm(note_msg: &ekreta::NoteMsg) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", note_msg.cim);
    _ = writeln!(&mut f, "| Időpont: {}", note_msg.datum.pretty());
    _ = writeln!(&mut f, "| {}", note_msg.keszito_tanar_neve);
    let rendered = nanohtml2text::html2text(&note_msg.tartalom_formazott);
    _ = write!(&mut f, "\n{rendered}",);
    f
}

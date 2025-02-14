//! messages from teachers and staff

use crate::config::Config;
use crate::Rendr;
use crate::{fill, paths::download_dir, time::MyDate, user::Usr};
use ekreta::Res;
use std::{char, fmt::Write};

pub fn handle(notes: bool, user: &Usr, rev: bool, num: usize, conf: &Config) -> Res<()> {
    if notes {
        let notes = user.get_note_msgs((None, None))?;
        if rev {
            for note in notes.iter().take(num).rev() {
                let as_str = disp_note_msg(note, conf.renderer);
                println!("\n\n\n\n{as_str}");
                fill(&as_str, '-', None);
            }
        } else {
            for note in notes.iter().take(num) {
                let as_str = disp_note_msg(note, conf.renderer);
                println!("\n\n\n\n{as_str}");
                fill(&as_str, '-', None);
            }
        }

        return Ok(());
    }
    let msgs = user.msgs((None, None))?;
    if rev {
        for msg in msgs.iter().rev().take(num) {
            let as_str = disp_msg(msg, conf.renderer);
            println!("\n\n\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    } else {
        for msg in msgs.iter().take(num) {
            let as_str = disp_msg(msg, conf.renderer);
            println!("\n\n\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    }
    Ok(())
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

pub fn disp_msg(msg: &ekreta::MsgItem, renderer: Rendr) -> String {
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
    _ = write!(
        &mut f,
        "\n{}",
        Rendr::render_html(&msg.uzenet.szoveg, Some(renderer)).trim()
    );
    // if !msg.is_elolvasva {
    //  writeln!(f, "Olvasatlan")?;
    // }
    f
}

pub fn disp_note_msg(note_msg: &ekreta::NoteMsg, renderer: Rendr) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", note_msg.cim);
    _ = writeln!(&mut f, "| Időpont: {}", note_msg.datum.pretty());
    _ = writeln!(&mut f, "| {}", note_msg.keszito_tanar_neve);
    _ = write!(
        &mut f,
        "\n{}",
        Rendr::render_html(&note_msg.tartalom_formazott, Some(renderer)).trim()
    );
    f
}

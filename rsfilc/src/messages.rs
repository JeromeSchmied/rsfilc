//! messages from teachers and staff

use crate::{paths::download_dir, time::MyDate};
use ekreta::{MsgKind, Res};
use std::{
    char,
    fmt::{self, Write},
    io::Read,
    process::{Child, Command, Stdio},
};

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
#[cfg(test)]
#[test]
fn fix_file_name() {
    let f = ekreta::Attachment {
        fajl_nev: "ain't a good filename is it ted? .txt .doksi .docx".to_string(),
        azonosito: 38,
    };
    assert_eq!(
        download_dir().join("ain't_a_good_filename_is_it_ted?_.txt_.doksi_.docx"),
        download_attachment_to(&f)
    );
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
    _ = write!(
        &mut f,
        "\n{}",
        Rendr::render_html(&msg.uzenet.szoveg).trim()
    );
    // if !msg.is_elolvasva {
    //  writeln!(f, "Olvasatlan")?;
    // }
    f
}

/// supported external programs that can render html
enum Rendr {
    W3m,
    Lynx,
}
impl Rendr {
    /// Returns the child process needed for this [`Rendr`].
    fn child(&self) -> Res<Child> {
        match self {
            Rendr::W3m => {
                let mut w3m_cmd = Command::new("w3m");
                Ok(w3m_cmd
                    .args([
                        "-I",
                        "utf-8",
                        "-T",
                        "text/html",
                        "-o",
                        "display_link=true",
                        "-o",
                        "display_link_number=true",
                        "-dump",
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?)
            }
            Rendr::Lynx => {
                let mut lynx_cmd = Command::new("lynx");
                Ok(lynx_cmd
                    .args([
                        "-stdin",
                        "-dump",
                        "-assume_charset",
                        "utf-8",
                        "-display_charset",
                        "utf-8",
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?)
            }
        }
    }
    /// Returns the other [`Rendr`].
    fn other(&self) -> Self {
        match self {
            Self::W3m => Self::Lynx,
            Self::Lynx => Self::W3m,
        }
    }
    /// Returns both [`Rendr`]'s name.
    fn both_name() -> &'static str {
        "w3m/lynx"
    }

    /// render html with a [`Rendr`]
    fn render_with_external(html: &str, pref: Option<Rendr>) -> Option<String> {
        let pref = if let Some(pr) = pref { pr } else { Rendr::Lynx };

        let proc = if let Ok(pref_proc) = pref.child() {
            // eprintln!("rendering with {}", pref);
            pref_proc
        } else if let Ok(other_proc) = pref.other().child() {
            // eprintln!("rendering with {}", pref.other());
            other_proc
        } else {
            eprintln!("couldn't spawn lynx nor w3m, falling back to very-very-basic-html-renderer-written-in-22-lines ;)");
            return None;
        };

        if let Err(why) = std::io::Write::write_all(&mut proc.stdin.unwrap(), html.as_bytes()) {
            eprintln!("couldn't write to {} stdin: {why}", Rendr::both_name());
            return None;
        };

        let mut ext_dump = String::new();
        if let Err(why) = proc.stdout.unwrap().read_to_string(&mut ext_dump) {
            eprintln!("couldn't read {} stdout: {why}", Rendr::both_name());
            return None;
        }

        Some(ext_dump.replace("\\\"", ""))
    }

    /// Very-Very Basic Html Renderer Written In 22 Lines Of Code: ˝render˝ html to console
    fn vvbhrwi22loc(html: &str) -> String {
        let html = html.replace('\\', "");

        let mut text = String::new();
        let mut is_attr = false;
        let mut attr = String::new();

        for ch in html.chars() {
            if ch == '<' {
                is_attr = true;
            } else if ch == '>' {
                is_attr = false;

                if attr.contains('/') {
                    text.push('\n');
                }
            }

            if is_attr {
                attr.push(ch);
            } else {
                attr.clear();

                text.push(ch);
            }
        }

        text.replace('>', "").replace("\n\n\n", "\n")
    }
    fn render_html(html: &str) -> String {
        let render_pref = None;

        if let Some(ext) = Self::render_with_external(html, render_pref) {
            ext
        } else {
            Self::vvbhrwi22loc(html)
        }
    }
}
impl fmt::Display for Rendr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Rendr::W3m => "w3m",
                Rendr::Lynx => "lynx",
            }
        )?;
        Ok(())
    }
}

pub fn disp_msgkind(msgkind: MsgKind) -> &'static str {
    match msgkind {
        MsgKind::Recv => "Beérkezett",
        MsgKind::Sent => "Elküldve",
        MsgKind::Del => "Törölve",
    }
}

pub fn disp_note_msg(note_msg: &ekreta::NoteMsg) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", note_msg.cim);
    _ = writeln!(&mut f, "| Időpont: {}", note_msg.datum.pretty());
    _ = writeln!(&mut f, "| {}", note_msg.keszito_tanar_neve);
    _ = write!(
        &mut f,
        "\n{}",
        Rendr::render_html(&note_msg.tartalom_formazott).trim()
    );
    f
}

//! messages from teachers and staff

use crate::{fill, paths::download_dir, time::MyDate, user::Usr};
use ekreta::Res;
use render_html::Rendr;
use std::{char, fmt::Write};

pub fn handle(notes: bool, user: &Usr, rev: bool, num: usize) -> Res<()> {
    if notes {
        let notes = user.get_note_msgs((None, None))?;
        if rev {
            for note in notes.iter().take(num).rev() {
                let as_str = disp_note_msg(note);
                println!("\n\n\n\n{as_str}");
                fill(&as_str, '-', None);
            }
        } else {
            for note in notes.iter().take(num) {
                let as_str = disp_note_msg(note);
                println!("\n\n\n\n{as_str}");
                fill(&as_str, '-', None);
            }
        }

        return Ok(());
    }
    let msgs = user.msgs((None, None))?;
    if rev {
        for msg in msgs.iter().rev().take(num) {
            let as_str = disp_msg(msg);
            println!("\n\n\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    } else {
        for msg in msgs.iter().take(num) {
            let as_str = disp_msg(msg);
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

mod render_html {
    use ekreta::Res;
    use std::io::Write;
    use std::process::{Child, Command, Stdio};
    use std::{fmt, io::Read};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    /// supported external programs that can render html
    pub enum Rendr {
        W3m,
        Lynx,
    }
    impl Rendr {
        const fn name(self) -> &'static str {
            match self {
                Rendr::W3m => "w3m",
                Rendr::Lynx => "lynx",
            }
        }
        fn args(self) -> &'static [&'static str] {
            match self {
                Rendr::W3m => &[
                    "-I",
                    "utf-8",
                    "-T",
                    "text/html",
                    "-o",
                    "display_link=true",
                    "-o",
                    "display_link_number=true",
                    "-dump",
                ],
                Rendr::Lynx => &[
                    "-stdin",
                    "-dump",
                    "-assume_charset",
                    "utf-8",
                    "-display_charset",
                    "utf-8",
                ],
            }
        }
    }

    impl Rendr {
        /// Returns the child process needed for this [`Rendr`].
        pub fn child(&self) -> Res<Child> {
            let mut cmd = Command::new(self.name());
            Ok(cmd
                .args(self.args())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?)
        }
        /// Returns the other [`Rendr`].
        pub fn other(&self) -> Self {
            match self {
                Self::W3m => Self::Lynx,
                Self::Lynx => Self::W3m,
            }
        }
        /// Returns both [`Rendr`]'s name.
        pub fn both_name() -> &'static str {
            "w3m/lynx"
        }

        /// render html with a [`Rendr`]
        pub fn render_with_external(html: &str, pref: Option<Rendr>) -> Option<String> {
            let pref = if let Some(pr) = pref { pr } else { Rendr::Lynx };

            let proc = if let Ok(pref_proc) = pref.child() {
                log::info!("rendering with {pref}");
                pref_proc
            } else if let Ok(other_proc) = pref.other().child() {
                log::info!("rendering with {}", pref.other());
                other_proc
            } else {
                eprintln!("couldn't spawn lynx nor w3m, falling back to very-very-basic-html-renderer-written-in-22-lines ;)");
                return None;
            };

            Write::write_all(&mut proc.stdin?, html.as_bytes())
                .inspect_err(|e| eprintln!("couldn't write to {} stdin: {e}", Rendr::both_name()))
                .ok()?;

            let mut ext_dump = String::new();
            if let Err(why) = proc.stdout.unwrap().read_to_string(&mut ext_dump) {
                eprintln!("couldn't read {} stdout: {why}", Rendr::both_name());
                return None;
            }

            Some(ext_dump.replace("\\\"", ""))
        }

        /// Very-Very Basic Html Renderer Written In 22 Lines Of Code: ˝render˝ html to console
        pub fn vvbhrwi22loc(html: &str) -> String {
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
        pub fn render_html(html: &str) -> String {
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

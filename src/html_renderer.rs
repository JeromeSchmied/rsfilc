use ekreta::Res;
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::{
    fmt,
    io::{Read, Write},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
/// supported external programs that can render html
pub enum Rendr {
    #[default]
    Lynx,
    W3m,
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
                "display_link_number=true",
                "-dump",
            ],
            Rendr::Lynx => &[
                "-stdin",
                "-dump",
                "-assume_charset=utf-8",
                "-display_charset=utf-8",
            ],
        }
    }
}

impl Rendr {
    /// Returns the child process needed for this [`Rendr`].
    fn child(&self) -> Res<Child> {
        let mut cmd = Command::new(self.name());
        Ok(cmd
            .args(self.args())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?)
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
    pub fn render_html(html: &str, pref: Option<Rendr>) -> String {
        Self::render_with_external(html, pref).unwrap_or(Self::vvbhrwi22loc(html))
    }
}

impl fmt::Display for Rendr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name())?;
        Ok(())
    }
}

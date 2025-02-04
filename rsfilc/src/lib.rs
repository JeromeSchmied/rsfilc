//! `RsFilc`: `Kréta` API and client

pub mod absences;
pub mod announced;
pub mod args;
pub mod cache;
pub mod endpoints;
pub mod evals;
pub mod information;
pub mod messages;
pub mod paths;
pub mod schools;
mod time;
pub mod timetable;
pub mod user;

// reexports
pub use args::{Args, Commands};
pub use ekreta::Absence;
pub use ekreta::AnnouncedTest;
pub use ekreta::Evaluation;
pub use ekreta::Lesson;
pub use ekreta::OptIrval;
pub use ekreta::Res;
pub use user::User;

/// Fill under `this` with many `with` [`char`]s, inlaying `hint` if any.
///
/// this:   "123456789" <- len: 9
/// with:   '#'
/// hint:   "bab" <- len: 3
///
/// so:     "123456789" <- len: 9
/// result: "12 bab 89" <- len: 9
pub fn fill(this: &str, with: char, hint: Option<&str>) {
    let longest = this.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let inlay_hint = if let Some(il_hint) = hint {
        [" ", il_hint, " "].concat()
    } else {
        "".to_owned()
    };

    let left = (longest - inlay_hint.chars().count()) / 2;
    println!(
        "{}{}{}",
        with.to_string().repeat(left),
        inlay_hint,
        with.to_string()
            .repeat(longest - left - inlay_hint.chars().count())
    );
}

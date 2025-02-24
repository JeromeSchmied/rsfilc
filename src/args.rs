//! CLI arguments

use clap::{Parser, Subcommand};
use log::info;

/// default number of entries to show
const NUM: usize = usize::MAX;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// enable verbose logging into the log file
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// show cache dir
    #[arg(long, default_value_t = false)]
    pub cache_dir: bool,
    /// show config path
    #[arg(long, default_value_t = false)]
    pub config_path: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// starts the Text User Interface
    Tui {},
    /// generate completions for <SHELL>
    Completions { shell: clap_complete::Shell },

    /// information about lessons, today by default
    #[clap(visible_alias = "tt")]
    Timetable {
        /// which day to show: `+n|n-` (`n` is the number of days added to today) or [YYYY-][MM-][DD]
        #[arg(default_value_t = crate::timetable::default_day(), value_parser = crate::timetable::parse_day)]
        day: chrono::NaiveDate,

        /// show current lesson if any
        #[arg(short, long, default_value_t = false)]
        current: bool,
    },

    /// evaluations/grades the user received
    #[clap(visible_alias = "e")]
    Evals {
        /// filter by `subject`
        #[arg(short, long)]
        subject: Option<String>,
        /// filter by `kind` eg. témazáró, or `title`
        #[arg(short, long)]
        filter: Option<String>,
        /// calculate average
        #[arg(short, long, default_value_t = false)]
        average: bool,
        /// reverse the output
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
        /// number of entries to show
        #[arg(short, long, default_value_t = NUM)]
        number: usize,
        /// ghost evals, requires `--average`
        ghost: Vec<u8>,
    },

    /// messages the user either received or sent
    #[clap(visible_alias = "msg")]
    Messages {
        /// number of entries to show
        #[arg(short, long, default_value_t = NUM)]
        number: usize,
        /// reverse the output
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
        /// show additional notes/system messages
        #[arg(long, default_value_t = false)]
        notes: bool,
    },

    /// information about lessons the user missed
    #[clap(visible_alias = "a")]
    Absences {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long, default_value_t = NUM)]
        number: usize,
        /// count the number of absences
        #[arg(short, long, default_value_t = false)]
        count: bool,
        /// reverse the output
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },

    /// information about forecoming exams/tests
    #[clap(visible_alias = "t")]
    Tests {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long, default_value_t = NUM)]
        number: usize,
        /// reverse the output
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
        /// show tests from the past as well
        #[arg(short, long, default_value_t = false)]
        past: bool,
    },

    /// managing users of this program, listing if nothing specified
    #[clap(visible_alias = "u")]
    User {
        /// the id or name of the user used for args
        userid: Option<String>,
        /// delete an existing account
        #[arg(short, long, default_value_t = false, requires = "userid")]
        delete: bool,
        /// create an existing account
        #[arg(short, long, default_value_t = false, requires = "userid")]
        create: bool,
        /// switch between existing accounts
        #[arg(short, long, default_value_t = false, requires = "userid")]
        switch: bool,
        /// print the cache directory for a user
        #[arg(long, default_value_t = false)]
        cache_dir: bool,
    },

    /// information about all schools in the `Kréta` database
    #[clap(visible_alias = "s")]
    Schools {
        /// search for school
        #[arg(short, long, name = "SCHOOL_PROPERTY")]
        search: Option<String>,
    },
}
impl Command {
    pub fn user_needed(&self) -> bool {
        info!("checking whether user is needed for task");
        if let Command::User {
            delete,
            create,
            switch,
            cache_dir,
            userid: _,
        } = &self
        {
            // we do need one on: nothing, switching, listing
            let nothing_specified = !delete && !create && !switch && !cache_dir;
            return nothing_specified || *switch;
        }
        !matches!(
            self,
            Command::Schools { search: _ } | Command::Completions { shell: _ }
        )
    }
}

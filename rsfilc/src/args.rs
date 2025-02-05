//! CLI arguments

use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

/// default number of entries to show
const NUM: usize = usize::MAX;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// starts the Text User Interface
    Tui {},

    /// generate shell completions
    Completions {
        /// the shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// information about lessons, today by default
    #[clap(visible_alias = "tt")]
    Timetable {
        /// which day to show: `name_of_day` or +n/n- (`n` is the number of days added to today) or YYYY/MM/DD
        day: Option<String>,

        /// show current lesson if any
        #[arg(short, long, default_value_t = false)]
        current: bool,

        /// export as json
        #[arg(short, long, name = "FILENAME.json")]
        export_day: Option<PathBuf>,
    },

    /// evaluations/grades the user recieved
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

    /// messages the user either recieved or sent
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

    /// managing users of this program
    #[clap(visible_alias = "u")]
    User {
        /// delete an existing account
        #[arg(short, long, default_value_t = false)]
        delete: bool,
        /// create an existing account
        #[arg(short, long, default_value_t = false)]
        create: bool,
        /// switch between existing accounts
        #[arg(short, long, name = "USERNAME")]
        switch: Option<String>,
        /// list all users
        #[arg(short, long, default_value_t = false)]
        list: bool,
    },

    /// information about all schools in the `Kréta` database
    #[clap(visible_alias = "s")]
    Schools {
        /// search for school
        #[arg(short, long, name = "SCHOOL_PROPERTY")]
        search: Option<String>,
    },
}
impl Commands {
    pub fn user_needed(&self) -> bool {
        info!("checking whether user is needed for task");
        if let Commands::User {
            delete,
            create,
            switch,
            list,
        } = &self
        {
            if !delete && !create && switch.is_none() && !list {
                return true;
            }
        }
        !matches!(
            self,
            Commands::Tui {}
                | Commands::Completions { shell: _ }
                | Commands::Schools { search: _ }
                | Commands::User {
                    delete: _,
                    create: _,
                    switch: _,
                    list: _
                }
        )
    }
}

//! CLI arugments

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
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
    Timetable {
        /// which day to show: +n (where n is day, and it's added to today) or YYYY/MM/DD
        #[arg(short, long)]
        day: Option<String>,

        /// show current lesson if any
        #[arg(short, long, default_value_t = false)]
        current: bool,
    },

    /// evaluations/grades the user recieved
    Evals {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// filter the kind to show
        #[arg(short, long)]
        kind: Option<String>,
        /// average
        #[arg(short, long, default_value_t = false)]
        average: bool,
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
        /// reverse direction of entries
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },

    /// messages the user either recieved or sent
    Messages {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
        /// reverse direction of entries
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },

    /// information about lessons the user missed
    Absences {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
        /// reverse direction of entries
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
        /// count the number of absences
        #[arg(short, long, default_value_t = false)]
        count: bool,
    },

    /// information about forecoming exams/tests
    Tests {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
        /// reverse direction of entries
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },

    /// managing users of this program
    User {
        /// delete an existing account
        #[arg(short, long, default_value_t = false)]
        delete: bool,
        /// create an existing account
        #[arg(short, long, default_value_t = false)]
        create: bool,
        /// switch between existing accounts
        #[arg(short, long)]
        switch: Option<String>,
        /// list all users
        #[arg(short, long, default_value_t = false)]
        list: bool,
    },

    /// information about all schools in the `Kréta` database
    Schools {
        /// search for school
        #[arg(short, long)]
        search: Option<String>,
    },
}
impl Commands {
    pub fn user_needed(&self) -> bool {
        !matches!(
            self,
            Commands::Tui {} | Commands::Completions { shell: _ } | Commands::Schools { search: _ }
        )
    }
}

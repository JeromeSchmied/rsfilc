//! CLI arguments

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::info;

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
    Timetable {
        /// which day to show: +n/n- (`n` is the number of days added to today) or YYYY/MM/DD
        #[arg(short, long)]
        day: Option<String>,

        /// show current lesson if any
        #[arg(short, long, default_value_t = false)]
        current: bool,

        /// export as json
        #[arg(short, long)]
        export_day: Option<PathBuf>,
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
        #[arg(short, long, default_value_t = usize::MAX)]
        number: usize,
    },

    /// messages the user either recieved or sent
    Messages {
        /// number of entries to show
        #[arg(short, long, default_value_t = usize::MAX)]
        number: usize,
    },

    /// information about lessons the user missed
    Absences {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long, default_value_t = usize::MAX)]
        number: usize,
        /// count the number of absences
        #[arg(short, long, default_value_t = false)]
        count: bool,
    },

    /// information about forecoming exams/tests
    Tests {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long, default_value_t = usize::MAX)]
        number: usize,
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
        info!("checking whether user is needed for task");
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

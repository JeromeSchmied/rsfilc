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

    /// information about lessons, today by default
    Timetable {
        /// which day to show
        #[arg(short, long)]
        day: Option<String>,
    },

    /// evaluations/grades the user recieved
    Evals {
        /// filter the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// filter the kind to show
        #[arg(short, long)]
        kind: Option<String>,
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
        /// average
        #[arg(short, long, default_value_t = false)]
        average: bool,
    },

    /// messages the user either recieved or sent
    Messages {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
    },

    /// information about lessons the user missed
    Absences {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
    },

    /// information about forecoming exams/tests
    Exams {
        /// number of entries to show
        #[arg(short, long, default_value_t = u16::MAX)]
        number: u16,
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

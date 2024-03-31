use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// starts the Text User Interface
    Tui {},

    /// information about lessons
    Timetable {
        /// which day to show
        #[arg(short, long)]
        day: Option<String>,
    },

    /// evaluations/grades the user recieved
    Evaluations {
        /// the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
    },

    /// messages the user either recieved or sent
    Messages {
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
    },

    /// information about lessons the user missed
    Absences {
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
    },

    /// information about forecoming exams/tests
    Exams {
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
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
        #[arg(short, long, default_value_t = false)]
        switch: bool,
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

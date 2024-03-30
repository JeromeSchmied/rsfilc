use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// starts Text User Interface
    Tui {},
    /// timetable
    TimeTable {
        /// which day to show
        #[arg(short, long)]
        time: Option<String>,
    },
    /// evaluations/grades
    Evaluations {
        /// the subject to show
        #[arg(short, long)]
        subject: Option<String>,
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
    },
    /// messages
    Messages {
        /// number of entries to show
        #[arg(short, long)]
        number: Option<u16>,
    },
    /// absences
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
    /// user
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
    },
}

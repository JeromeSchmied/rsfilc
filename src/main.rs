use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Offset, Timelike, Utc};
use clap::Parser;
use rsfilc::{
    api::*, api_urls::ApiUrls, args::Commands, messages::MessageKind, school_list::School,
    timetable, AnyErr,
};

#[tokio::main]
async fn main() -> AnyErr<()> {
    let cli_args = rsfilc::args::Args::parse();

    let user = if let Some(loaded_user) = User::load() {
        loaded_user
    } else {
        User::create()
    };
    user.greet().await;
    // let user = User::new(username, password, school_id);

    match cli_args.command {
        Some(cm) => match cm {
            Commands::Tui {} => todo!("TUI is to be written (soon)"),
            Commands::TimeTable { time } => {
                let day = if let Some(date) = time {
                    NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                        .expect("couldn't parse date got from user")
                    // DateTime::parse_from_str(&date, "%Y-%m-%d")
                    //     .expect("couldn't parse date")
                    //     .into()
                } else {
                    Utc::now().date_naive()
                };
                // let date = "2024-03-27";
                let from = day.and_hms_opt(0, 0, 0).expect("couldn't make from");
                let to = day.and_hms_opt(23, 59, 59).expect("couldn't make from");
                // let from = DateTime::parse_from_rfc2822(&format!("{}T00:00Z", date)).expect("invalid date-time");
                // let to = DateTime::parse_from_str(&format!("{}T18:00Z", date)).expect("invalid date-time");
                // let from = DateTime::parse_str("2024-03-22T00:00").expect("invalid date-time");
                // let to = DateTime::parse_str("2024-03-22T23:59").expect("invalid date-time");
                // let from = DateTime::now(0).expect("invalid date-time");
                // let from = DateTime::now(0).expect("invalid date-time");
                let mut timetable = user.timetable(from, to).await?;
                eprintln!("\ngot timetable...\n");
                timetable
                    .sort_by(|a, b| a.from().partial_cmp(&b.from()).expect("couldn't compare"));
                timetable::Lesson::print_day(timetable);
                // println!("{}", timetable);
            }

            Commands::Evaluations { subject, number } => {
                let evals = user.evals().await?;
                eprintln!("\ngot evals...\n");
                println!("{}", evals);
            }
            Commands::Messages { number } => {
                let messages = user.messages(MessageKind::Beerkezett).await?;
                eprintln!("\ngot messages...\n");
                println!("{}", messages);
            }
            Commands::Absences { number } => {
                let absences = user.absencies().await?;
                eprintln!("\ngot absences...\n");
                println!("{}", absences);
            }
            Commands::Exams { number } => {
                let announced = user.announced(None).await?;
                eprintln!("\ngot announced...\n");
                println!("{}", announced);
            }
            Commands::User {
                delete,
                create,
                switch,
            } => {
                if delete {
                    todo!("user deletion is not yet ready");
                } else if create {
                    User::create();
                } else if switch {
                    todo!("switching between accounts is not yet ready");
                } else {
                    println!("{}", user.info().await?);
                }
            }
            Commands::Schools { search } => {
                let schools = School::get_from_refilc().await?;
                eprintln!("\ngot schools...\n");
                if let Some(school_name) = search {
                    let found = School::search(&school_name, &schools);
                    for school in found {
                        println!("{}\n", school);
                    }
                } else {
                    for school in schools {
                        println!("{}\n", school);
                    }
                }
            }
        },
        None => println!("showing timetable"),
    }

    // let apiurls = ApiUrls::api_urls().await?;
    // eprintln!("\ngot api urls...\n");
    // println!("{:#?}", apiurls);

    // let access_token = user.token().await?;
    // eprintln!("\ngot access_token...\n");
    // println!("{:?}", access_token);

    Ok(())
}

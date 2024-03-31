#![allow(unused)]

use chrono::{
    DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Timelike, Utc,
};
use clap::Parser;
use rsfilc::{
    api::*, api_urls::ApiUrls, args::Commands, messages::MessageKind, school_list::School,
    timetable, AnyErr,
};

#[tokio::main]
async fn main() -> AnyErr<()> {
    let cli_args = rsfilc::args::Args::parse();

    let users = User::load_all();

    let user = if let Some(default_user) = User::load_conf().await {
        default_user
    } else if let Some(loaded_user) = users.first() {
        loaded_user.clone()
    } else {
        User::create()
    };
    user.greet().await;
    // let user = User::new(username, password, school_id);

    match cli_args.command {
        Commands::Tui {} => todo!("TUI is to be written (soon)"),
        Commands::Timetable { day } => {
            let day = if let Some(date) = day {
                NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                    .expect("couldn't parse date got from user")
            } else {
                Local::now().date_naive()
            };
            let from = day
                .and_hms_opt(0, 0, 0)
                .expect("couldn't make from")
                .and_local_timezone(Local)
                .unwrap();
            let to = day
                .and_hms_opt(23, 59, 59)
                .expect("couldn't make from")
                .and_local_timezone(Local)
                .unwrap();
            let mut lessons = user.timetable(from, to).await?;
            if lessons.is_empty() {
                println!("Ezen a napon {} ({}) nincs órád, juhé!", day, day.weekday());
                return Ok(());
            }

            // eprintln!("\ngot timetable...\n");
            lessons.sort_by(|a, b| a.from().partial_cmp(&b.from()).expect("couldn't compare"));
            timetable::Lesson::print_day(&lessons);
        }

        Commands::Evals { subject, number } => {
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
            list,
        } => {
            if let Some(switch_to) = switch {
                let usr = User::load_user(&switch_to).await.unwrap();
                println!("switched to {}", switch_to);
                usr.greet().await;

                return Ok(());
            }
            if delete {
                todo!("user deletion is not yet ready");
            } else if create {
                User::create();
            } else if list {
                println!("\nFelhasználók:\n");
                for current_user in users {
                    println!("{}\n", current_user.info().await?);
                }
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
    }

    // let apiurls = ApiUrls::api_urls().await?;
    // eprintln!("\ngot api urls...\n");
    // println!("{:#?}", apiurls);

    // let access_token = user.token().await?;
    // eprintln!("\ngot access_token...\n");
    // println!("{:?}", access_token);

    Ok(())
}

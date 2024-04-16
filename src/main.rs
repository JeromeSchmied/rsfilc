use chrono::{Datelike, Duration, Local, NaiveDate};
use clap::{CommandFactory, Parser};
use rsfilc::{
    args::{Args, Commands},
    evals::Eval,
    log_path,
    school_list::School,
    timetable,
    user::User,
    AnyErr,
};
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> AnyErr<()> {
    let cli_args = Args::parse();

    let user = if cli_args.command.user_needed() {
        let users = User::load_all();
        if let Some(default_user) = User::load_conf().await {
            default_user
        } else if let Some(loaded_user) = users.first() {
            loaded_user.clone()
        } else {
            User::create()
        }
    } else {
        User::new("", "", "") // dummy user
    };

    match cli_args.command {
        Commands::Tui {} => todo!("TUI is to be written (soon)"),
        Commands::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Args::command(),
                "rsfilc",
                &mut std::io::stdout(),
            );
        }
        Commands::Timetable { day, current } => {
            if current {
                if let Some(current_lessons) = user.current_lesson().await {
                    for current_lesson in current_lessons {
                        println!(
                            "{}, {}m",
                            current_lesson.subject(),
                            (current_lesson.end() - Local::now()).num_minutes()
                        );
                    }
                }
                return Ok(());
            }
            let day = if let Some(date) = day {
                let date = date.replace(['/', '.'], "-");
                if let Ok(ndate) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    ndate
                } else if date.starts_with('+') {
                    Local::now()
                        .checked_add_signed(Duration::days(
                            date.parse::<i64>().expect("invalid day shifter"),
                        ))
                        .expect("invalid datetime")
                        .date_naive()
                } else {
                    Local::now().date_naive()
                }
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
                println!(
                    "Ezen a napon {day} ({}) nincs rögzített órád, juhé!",
                    day.weekday()
                );
                return Ok(());
            }

            // eprintln!("\ngot timetable...\n");
            lessons.sort_by(|a, b| a.start().partial_cmp(&b.start()).expect("couldn't compare"));
            timetable::Lesson::print_day(&lessons);
        }

        Commands::Evals {
            subject,
            kind,
            number,
            average,
            reverse,
        } => {
            let mut evals = user.evals(None, None).await?;
            if !reverse {
                evals.sort_by(|a, b| {
                    b.earned()
                        .partial_cmp(&a.earned())
                        .expect("couldn't compare")
                });
            }
            // eprintln!("\ngot evals...\n");
            if let Some(kind) = kind {
                Eval::filter_evals_by_kind(&mut evals, &kind);
            }
            if let Some(subject) = subject {
                Eval::filter_evals_by_subject(&mut evals, &subject);
            }
            let mut logf = File::create(log_path("evals_filtered"))?;
            write!(logf, "{:?}", evals)?;

            if average {
                println!("Average: {}", Eval::average(&evals));

                return Ok(());
            }

            for eval in evals.iter().take(number.into()) {
                println!("{}", eval);
            }
        }

        Commands::Messages { number, reverse } => {
            let mut msg_overviews = user.all_msg_oviews().await?;
            if !reverse {
                msg_overviews
                    .sort_by(|a, b| b.sent().partial_cmp(&a.sent()).expect("couldn't compare"));
            }

            for msg_overview in msg_overviews.iter().take(number.into()) {
                let full_msg = user.full_msg(msg_overview).await?;
                // println!("{}", msg_overview);
                println!("{}", full_msg);
            }
        }

        Commands::Absences {
            number,
            count,
            reverse,
        } => {
            let mut absences = user.absences(None, None).await?;
            if count {
                println!("Összes hiányzásod száma: {}", absences.len());
                println!(
                    "Ebből még igazolatlan: {}",
                    absences.iter().filter(|item| !item.verif()).count()
                );
                return Ok(());
            }

            if !reverse {
                absences
                    .sort_by(|a, b| b.start().partial_cmp(&a.start()).expect("couldn't compare"));
            }

            for absence in absences.iter().take(number.into()) {
                println!("{}", absence);
            }
        }

        Commands::Tests { number, reverse } => {
            let mut all_announced = user.all_announced(None).await?;
            if !reverse {
                all_announced
                    .sort_by(|a, b| b.day().partial_cmp(&a.day()).expect("couldn't compare"));
            }

            for announced in all_announced.iter().take(number.into()) {
                println!("{}", announced);
            }
        }

        Commands::User {
            delete,
            create,
            switch,
            list,
        } => {
            if let Some(switch_to) = switch {
                let switched_to = User::load_user(&switch_to).await.unwrap();
                println!("switched to {switch_to}");
                switched_to.greet().await;

                return Ok(());
            }
            if delete {
                todo!("user deletion is not yet ready");
            } else if create {
                User::create();
            } else if list {
                println!("\nFelhasználók:\n");
                for current_user in User::load_all() {
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
                    println!("{school}\n");
                }
            } else {
                for school in schools {
                    println!("{school}\n");
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

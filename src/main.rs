use chrono::{Datelike, Local};
use clap::{CommandFactory, Parser};
use log::*;
use rsfilc::{
    args::{Args, Commands},
    evals::Eval,
    log_file,
    school_list::School,
    timetable::Lesson,
    user::User,
    AnyErr,
};
use simplelog::{LevelFilter, WriteLogger};
use std::io::Write;

fn main() -> AnyErr<()> {
    // set up logger
    WriteLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        log_file("rsfilc")?,
    );

    let cli_args = Args::parse();

    let user = if cli_args.command.user_needed() {
        let users = User::load_all();
        if let Some(default_user) = User::load_conf() {
            default_user
        } else if let Some(loaded_user) = users.first() {
            loaded_user.clone()
        } else {
            User::create()
        }
    } else {
        info!(
            "created dummy user, as it's not needed for {:?} command",
            cli_args.command
        );
        User::new("", "", "") // dummy user
    };

    let now = Local::now();

    match cli_args.command {
        Commands::Tui {} => {
            warn!("TUI is not yet written");
            todo!("TUI is to be written (soon)")
        }
        Commands::Completions { shell } => {
            info!("creating shell completions for {}", shell);
            clap_complete::generate(
                shell,
                &mut Args::command(),
                "rsfilc",
                &mut std::io::stdout(),
            );
        }
        Commands::Timetable { day, current } => {
            if current {
                for current_lesson in user.current_lessons() {
                    println!(
                        "{}, {}m",
                        current_lesson.subject(),
                        (current_lesson.end() - now).num_minutes() // minutes remaining
                    );
                }
                return Ok(());
            }

            // parse day
            let day = Lesson::parse_day(&day);

            let from = day
                .and_hms_opt(0, 0, 0)
                .expect("couldn't make from")
                .and_local_timezone(Local)
                .unwrap();
            let to = day
                .and_hms_opt(23, 59, 59)
                .expect("couldn't make to")
                .and_local_timezone(Local)
                .unwrap();

            let lessons = user.timetable(from, to)?;
            if lessons.is_empty() {
                println!(
                    "Ezen a napon {day} ({}) nincs rögzített órád, juhé!",
                    day.weekday()
                );
                return Ok(());
            }

            Lesson::print_day(&lessons);
        }

        Commands::Evals {
            subject,
            kind,
            number,
            average,
        } => {
            let mut evals = user.evals(None, None)?;
            info!("got evals");
            if let Some(kind) = kind {
                Eval::filter_by_kind(&mut evals, &kind);
            }
            if let Some(subject) = subject {
                Eval::filter_by_subject(&mut evals, &subject);
            }

            let mut logf = log_file("evals_filtered")?;
            write!(logf, "{:?}", evals)?;

            if average {
                println!("Average: {}", Eval::average(&evals));

                return Ok(());
            }

            for eval in evals.iter().take(number) {
                println!("{eval}");
            }
        }

        Commands::Messages { number } => {
            for msg_oview in user.all_msg_oviews()?.iter().take(number) {
                let full_msg = user.full_msg(msg_oview)?;
                // println!("{}", msg_overview);
                println!("{}", full_msg);
                user.download_attachments(&full_msg)?;
            }
        }

        Commands::Absences { number, count } => {
            let absences = user.absences(None, None)?;
            if count {
                println!("Összes hiányzásod száma: {}", absences.len());
                println!(
                    "Ebből még igazolatlan: {}",
                    absences.iter().filter(|item| !item.verif()).count()
                );
                return Ok(());
            }

            for absence in absences.iter().take(number) {
                println!("{}", absence);
            }
        }

        Commands::Tests { number } => {
            for announced in user.all_announced(None)?.iter().take(number) {
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
                let switched_to = User::load(&switch_to).expect("couldn't load user");
                info!("switched to user {switch_to}");
                println!("switched to {switch_to}");
                println!("Hello {}!", switched_to.name());

                return Ok(());
            }
            if delete {
                todo!("user deletion is not yet ready");
            } else if create {
                User::create();
            } else if list {
                println!("\nFelhasználók:\n");
                for current_user in User::load_all() {
                    println!("{}\n", current_user.info()?);
                }
            } else {
                println!("{}", user.info()?);
            }
        }

        Commands::Schools { search } => {
            let schools = School::get_from_refilc()?;
            if let Some(school_name) = search {
                let found = School::search(&school_name, &schools);
                for school in found {
                    println!("{school}\n");
                }
            } else {
                info!("listing schools");
                for school in schools {
                    println!("{school}\n");
                }
            }
        }
    }

    // let apiurls = ApiUrls::api_urls()?;
    // eprintln!("\ngot api urls...\n");
    // println!("{:#?}", apiurls);

    // let access_token = user.token()?;
    // eprintln!("\ngot access_token...\n");
    // println!("{:?}", access_token);

    Ok(())
}

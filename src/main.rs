use chrono::{Datelike, Local};
use clap::{CommandFactory, Parser};
use log::*;
use rsfilc::{Abs, Ancd, Eval, School, User, *};
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() -> Res<()> {
    // set up logger
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {}",
                Local::now(),
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path("rsfilc"))?,
        )
        // Apply globally
        .apply()?;

    // parse
    let cli_args = Args::parse();

    // have a valid user
    let user = if cli_args.command.user_needed() {
        let users = User::load_all(); // load every saved user
        if let Some(default_user) = User::load_conf() {
            default_user // if specified, load preferred user
        } else if let Some(loaded_user) = users.first() {
            loaded_user.clone() // load first user
        } else {
            User::create() // create a new user
        }
    } else {
        info!(
            "created dummy user, as it's not needed for {:?} command",
            cli_args.command
        );
        User::new("", "", "") // dummy user
    };

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
        Commands::Timetable {
            day,
            current,
            export_day,
        } => {
            // parse day
            let day = timetable::parse_day(&day);

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

            if current {
                let current_lessons = timetable::current_lessons(&lessons);
                info!("current lessons: {:?}", current_lessons);
                if current_lessons.is_empty() {
                    if let Some(nxt) = timetable::next_lesson(&lessons) {
                        println!(
                            "{}m -> {}",
                            (nxt.start() - Local::now()).num_minutes(), // minutes remaining
                            nxt.subject()
                        );
                    }
                }
                for current_lesson in current_lessons {
                    println!(
                        "{}, {}m",
                        current_lesson.subject(),
                        (current_lesson.end() - Local::now()).num_minutes() // minutes remaining
                    );
                }

                return Ok(());
            }

            if let Some(export_json_to) = export_day {
                info!("exported timetable to json");
                let mut f = File::create(export_json_to)?;
                let content = serde_json::to_string(&lessons)?;
                write!(f, "{}", content)?;
            }

            user.print_day(&lessons);
        }

        Commands::Evals {
            subject,
            kind,
            number,
            average,
            reverse,
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

            if !reverse {
                for eval in evals.iter().take(number) {
                    println!("{eval}");
                }
            } else {
                for eval in evals.iter().take(number).rev() {
                    println!("{eval}");
                }
            }
        }

        Commands::Messages { number, reverse } => {
            let msgs = user.msgs(None, None)?;

            if !reverse {
                for msg in msgs.iter().take(number) {
                    println!("{msg}");
                    user.download_attachments(msg)?;
                }
            } else {
                for msg in msgs.iter().take(number).rev() {
                    println!("{msg}");
                    user.download_attachments(msg)?;
                }
            }
        }

        Commands::Absences {
            number,
            count,
            subject,
            reverse,
        } => {
            let mut absences = user.absences(None, None)?;
            if let Some(subject) = subject {
                Abs::filter_by_subject(&mut absences, &subject);
            }

            if count {
                println!("Összes hiányzásod száma: {}", absences.len());
                println!(
                    "Ebből még igazolatlan: {}",
                    absences.iter().filter(|item| !item.verif()).count()
                );
                return Ok(());
            }

            if !reverse {
                for absence in absences.iter().take(number) {
                    println!("{}", absence);
                }
            } else {
                for absence in absences.iter().take(number).rev() {
                    println!("{}", absence);
                }
            }
        }

        Commands::Tests {
            number,
            subject,
            reverse,
        } => {
            let mut all_announced = user.all_announced(None, None)?;
            if let Some(subject) = subject {
                Ancd::filter_by_subject(&mut all_announced, &subject);
            }

            if !reverse {
                for announced in all_announced.iter().take(number) {
                    println!("{}", announced);
                }
            } else {
                for announced in all_announced.iter().take(number).rev() {
                    println!("{}", announced);
                }
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
                    println!("{}", current_user.info()?);
                    println!("---------------------------\n");
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
                    println!("{school}");
                    println!("\n---------------------------\n");
                }
            } else {
                info!("listing schools");
                for school in schools {
                    println!("{school}");
                    println!("\n---------------------------\n");
                }
            }
        }
    }

    Ok(())
}

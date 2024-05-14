use chrono::Local;
use clap::{CommandFactory, Parser};
use log::*;
use rsfilc::{Abs, Ancd, Eval, School, User, *};
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() -> Res<()> {
    // set up fern
    set_up_logger()?;

    // parse args
    let cli_args = Args::parse();

    // have a valid user
    let user = create_user(&cli_args)?;

    // handle cli args and execute program
    run(cli_args, &user)?;

    Ok(())
}

fn run(cli_args: Args, user: &User) -> Res<()> {
    match cli_args.command {
        Commands::Tui {} => {
            warn!("TUI is not yet written");
            todo!("TUI is to be written (soon)");
        }
        Commands::Completions { shell } => {
            info!("creating shell completions for {shell}");
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

            let day_start = day
                .and_hms_opt(0, 0, 0)
                .expect("couldn't make from")
                .and_local_timezone(Local)
                .unwrap();
            let day_end = day
                .and_hms_opt(23, 59, 59)
                .expect("couldn't make to")
                .and_local_timezone(Local)
                .unwrap();

            let lessons = user.fetch_timetable(day_start, day_end)?;

            // nice output if no lessons, couldn't be possible in print_day()
            if lessons.is_empty() {
                println!(
                    "{} ({}) nincs rögzített órád, juhé!",
                    day_start.pretty(),
                    day_start.hun_day_of_week()
                );
                return Ok(());
            }

            if current {
                let current_lessons = timetable::current_lessons(&lessons);
                if current_lessons.is_empty() {
                    if let Some(nxt) = timetable::next_lesson(&lessons) {
                        println!(
                            "{}m -> {}",
                            (nxt.start() - Local::now()).num_minutes(), // minutes remaining
                            nxt.subject
                        );
                    }
                }
                for current_lesson in current_lessons {
                    println!(
                        "{}, {}m",
                        current_lesson.subject,
                        (current_lesson.end() - Local::now()).num_minutes() // minutes remaining
                    );
                }

                return Ok(());
            }

            if let Some(export_json_to) = export_day {
                info!("exported timetable to json");
                let mut f = File::create(export_json_to)?;
                let content = serde_json::to_string(&lessons)?;
                write!(f, "{content}")?;
            }

            user.print_day(&lessons);
        }

        Commands::Evals {
            subject,
            filter,
            number,
            average,
            reverse,
            ghost,
        } => {
            let mut evals = user.fetch_evals((None, None))?;
            info!("got evals");
            if let Some(kind) = filter {
                Eval::filter_by_kind_or_title(&mut evals, &kind);
            }
            if let Some(subject) = subject {
                Eval::filter_by_subject(&mut evals, &subject);
            }

            let mut logf = log_file("evals_filtered")?;
            write!(logf, "{evals:?}")?;

            // ghost without average has no effect
            if !ghost.is_empty() && !average {
                return Err("Oyy! Didn't I tell you to use `ghost` with `average`?".into());
            }

            if average {
                let avg = Eval::average(&evals, &ghost);
                println!("Average: {avg}");

                return Ok(());
            }

            if reverse {
                for eval in evals.iter().take(number).rev() {
                    print!("\n\n{eval}");
                    fill_under(&eval.to_string(), '-');
                }
            } else {
                for eval in evals.iter().take(number) {
                    print!("\n\n{eval}");
                    fill_under(&eval.to_string(), '-');
                }
            }
        }

        Commands::Messages {
            number,
            reverse,
            notes,
        } => {
            if notes {
                let notes = user.fetch_note_msgs()?;
                if reverse {
                    for note in notes.iter().take(number).rev() {
                        println!("\n\n\n\n{note}");
                        fill_under(&note.to_string(), '-');
                    }
                } else {
                    for note in notes.iter().take(number) {
                        println!("\n\n\n\n{note}");
                        fill_under(&note.to_string(), '-');
                    }
                }

                return Ok(());
            }

            let msgs = user.msgs((None, None), Some(number))?;
            if reverse {
                for msg in msgs.iter().rev() {
                    println!("\n\n\n\n{msg}");
                    fill_under(&msg.to_string(), '-');
                    user.download_attachments(msg)?;
                }
            } else {
                for msg in &msgs {
                    println!("\n\n\n\n{msg}");
                    fill_under(&msg.to_string(), '-');
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
            let mut absences = user.fetch_absences((None, None))?;
            if let Some(subject) = subject {
                Abs::filter_by_subject(&mut absences, &subject);
            }

            if count {
                println!("Összes hiányzásod száma: {}", absences.len());
                println!(
                    "Ebből még igazolatlan: {}",
                    absences.iter().filter(|item| !item.verified()).count()
                );
                return Ok(());
            }

            if reverse {
                for absence in absences.iter().take(number).rev() {
                    print!("\n\n{absence}");
                    fill_under(&absence.to_string(), '-');
                }
            } else {
                for absence in absences.iter().take(number) {
                    print!("\n\n{absence}");
                    fill_under(&absence.to_string(), '-');
                }
            }
        }

        Commands::Tests {
            number,
            subject,
            reverse,
            past,
        } => {
            let from = if past { None } else { Some(Local::now()) };
            let mut all_announced = user.fetch_all_announced((from, None))?;
            if let Some(subject) = subject {
                Ancd::filter_by_subject(&mut all_announced, &subject);
            }

            if reverse {
                for announced in all_announced.iter().take(number).rev() {
                    print!("\n\n{announced}");
                    fill_under(&announced.to_string(), '-');
                }
            } else {
                for announced in all_announced.iter().take(number) {
                    print!("\n\n{announced}");
                    fill_under(&announced.to_string(), '-');
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
                println!("Hello {}!", switched_to.name()?);

                return Ok(());
            }
            if delete {
                todo!("user deletion is not yet ready");
            } else if create {
                User::create();
            } else if list {
                println!("\nFelhasználók:\n");
                for current_user in User::load_all() {
                    let user_info = current_user.fetch_info()?;
                    print!("\n\n{user_info}");
                    fill_under(&user_info.to_string(), '-');
                }
            } else {
                println!("{}", user.fetch_info()?);
            }
        }

        Commands::Schools { search } => {
            let schools = School::get_from_refilc()?;
            if let Some(school_name) = search {
                let found = School::search(&schools, &school_name);
                for school in found {
                    print!("\n\n{school}");
                    fill_under(&school.to_string(), '-');
                }
            } else {
                info!("listing schools");
                for school in schools {
                    print!("\n\n{school}");
                    fill_under(&school.to_string(), '-');
                }
            }
        }
    };
    Ok(())
}

fn create_user(cli_args: &Args) -> Res<User> {
    if cli_args.command.user_needed() {
        let users = User::load_all(); // load every saved user
        if let Some(default_user) = User::load_conf() {
            Ok(default_user) // if specified, load preferred user
        } else if let Some(loaded_user) = users.first() {
            Ok(loaded_user.clone()) // load first user
        } else if let Some(created) = User::create() {
            Ok(created)
        } else {
            return Err("couldn't find valid user".into());
        }
    } else {
        info!(
            "created dummy user, as it's not needed for {:?}",
            cli_args.command
        );
        Ok(User::dummy()) // dummy user
    }
}

fn set_up_logger() -> Res<()> {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {message}",
                Local::now(),
                record.level(),
                record.target(),
            ));
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
    Ok(())
}

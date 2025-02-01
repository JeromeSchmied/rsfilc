use chrono::{Datelike, Local};
use clap::{CommandFactory, Parser};
use log::*;
use rsfilc::{Absence, AnnouncedTest, Eval, School, User, *};
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

            let all_lessons_till_day = user.get_timetable(day, true)?;
            let lessons = user.get_timetable(day, false)?;

            // nice output if no lessons, couldn't be possible in print_day()
            if lessons.is_empty() {
                println!("{} ({}) nincs rögzített órád, juhé!", day, day.weekday());
                return Ok(());
            }

            if current {
                let current_lessons = timetable::current_lessons(&lessons);
                if let Some(nxt) = timetable::next_lesson(&all_lessons_till_day) {
                    println!(
                        "{}m -> {}",
                        (nxt.start - Local::now()).num_minutes(), // minutes remaining
                        nxt.subject
                    );
                }
                for current_lesson in current_lessons {
                    println!(
                        "{}, {}m",
                        current_lesson.subject,
                        (current_lesson.end - Local::now()).num_minutes() // minutes remaining
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
                println!("Average: {avg:.2}");

                return Ok(());
            }

            if reverse {
                for eval in evals.iter().take(number).rev() {
                    println!("\n\n{eval}");
                    fill(&eval.to_string(), '-', None);
                }
            } else {
                for eval in evals.iter().take(number) {
                    println!("\n\n{eval}");
                    fill(&eval.to_string(), '-', None);
                }
            }
        }

        Commands::Messages {
            number,
            reverse,
            notes,
        } => {
            if notes {
                let notes = user.fetch_note_msgs((None, None))?;
                if reverse {
                    for note in notes.iter().take(number).rev() {
                        println!("\n\n\n\n{note}");
                        fill(&note.to_string(), '-', None);
                    }
                } else {
                    for note in notes.iter().take(number) {
                        println!("\n\n\n\n{note}");
                        fill(&note.to_string(), '-', None);
                    }
                }

                return Ok(());
            }

            let msgs = user.msgs((None, None))?;
            if reverse {
                for msg in msgs.iter().rev().take(number) {
                    println!("\n\n\n\n{msg}");
                    fill(&msg.to_string(), '-', None);
                }
            } else {
                for msg in msgs.iter().take(number) {
                    println!("\n\n\n\n{msg}");
                    fill(&msg.to_string(), '-', None);
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
                absences::filter_by_subject(&mut absences, &subject);
            }

            if count {
                println!("Összes hiányzásod száma: {}", absences.len());
                println!(
                    "Ebből még igazolatlan: {}",
                    absences.iter().filter(|item| !item.igazolt()).count()
                );
                return Ok(());
            }

            if reverse {
                for absence in absences.iter().take(number).rev() {
                    let as_str = absences::disp(absence);
                    println!("\n\n{as_str}");
                    fill(&as_str, '-', None);
                }
            } else {
                for absence in absences.iter().take(number) {
                    let as_str = absences::disp(absence);
                    println!("\n\n{as_str}");
                    fill(&as_str, '-', None);
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
                announced::filter_by_subject(&mut all_announced, &subject);
            }

            if reverse {
                for announced in all_announced.iter().take(number).rev() {
                    let as_str = announced::disp(announced);
                    println!("\n\n{as_str}");
                    fill(&as_str, '-', None);
                }
            } else {
                for announced in all_announced.iter().take(number) {
                    let as_str = announced::disp(announced);
                    println!("\n\n{as_str}");
                    fill(&as_str, '-', None);
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
                delete_cache_dir()?;
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
                    println!("\n\n{user_info}");
                    fill(&user_info.to_string(), '-', None);
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
                    println!("\n\n{school}");
                    fill(&school.to_string(), '-', None);
                }
            } else {
                info!("listing schools");
                for school in schools {
                    println!("\n\n{school}");
                    fill(&school.to_string(), '-', None);
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

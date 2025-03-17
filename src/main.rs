use args::{Args, Command};
use chrono::Local;
use clap::{CommandFactory, Parser};
use config::Config;
use ekreta::Res;
use log::*;
use paths::cache_dir;
use std::fs::OpenOptions;
use user::Usr;
use utils::fill;

mod absences;
mod announced;
mod args;
mod cache;
mod config;
mod evals;
mod information;
mod messages;
mod paths;
mod schools;
mod time;
mod timetable;
mod user;
mod utils;

fn main() -> Res<()> {
    // parse args
    let cli_args = Args::parse();
    // set up fern
    set_up_logger(cli_args.verbose)?;
    // load config from file, eg.: users
    let mut config = Config::load()?;

    // handle cli args and execute program
    run(cli_args, &mut config)?;

    Ok(())
}

fn run(args: Args, conf: &mut Config) -> Res<()> {
    if args.command.is_none() {
        if args.cache_dir {
            println!("{}", cache_dir("").ok_or("no cache dir found")?.display());
            return Ok(());
        }
        if args.config_path {
            println!("{}", Config::path()?.display());
            return Ok(());
        }
    }
    let command = args.command.unwrap_or(Command::Timetable {
        day: None,
        current: false,
    });
    // have a valid user
    let user = if command.user_needed() {
        Usr::load(conf).ok_or("no user found, please create one with `rsfilc user --create`")?
    } else {
        Usr::dummy()
    };

    match command {
        Command::Completions { shell: sh } => {
            info!("creating shell completions for {sh}");
            clap_complete::generate(sh, &mut Args::command(), "rsfilc", &mut std::io::stdout());
            return Ok(());
        }
        Command::Tui {} => {
            warn!("TUI is not yet written");
            todo!("TUI is to be written (soon)");
        }
        Command::Timetable { day, current } => {
            timetable::handle(day, &user, current)?;
        }

        Command::Evals {
            subject,
            filter,
            number,
            average,
            reverse,
            ghost,
        } => {
            evals::handle(&user, filter, subject, &ghost, average, reverse, number)?;
        }

        Command::Messages {
            number,
            reverse,
            notes,
        } => {
            messages::handle(notes, &user, reverse, number)?;
        }

        Command::Absences {
            number,
            count,
            subject,
            reverse,
        } => {
            absences::handle(&user, subject, count, reverse, number)?;
        }

        Command::Tests {
            number,
            subject,
            reverse,
            past,
        } => {
            announced::handle(past, &user, subject, reverse, number)?;
        }

        Command::User {
            delete,
            create,
            switch,
            cache_dir,
            userid,
        } => {
            user::handle(userid, create, conf, delete, switch, cache_dir)?;
        }

        Command::Schools { search } => {
            schools::handle(search)?;
        }
    }
    Ok(())
}

fn set_up_logger(verbose: bool) -> Res<()> {
    let path = paths::cache_dir("")
        .ok_or("no cache dir")?
        .join(config::APP_NAME)
        .with_extension("log");
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {message}",
                Local::now(),
                record.level(),
                record.target(),
            ));
        })
        .level(if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(OpenOptions::new().create(true).append(true).open(path)?)
        .apply()?;
    eprintln!("set upt logger");
    Ok(())
}

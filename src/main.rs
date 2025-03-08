use args::{Args, Command};
use chrono::Local;
use clap::{CommandFactory, Parser};
use config::Config;
use ekreta::Res;
use log::*;
use paths::cache_dir;
use std::fs::OpenOptions;
use std::io::{self, IsTerminal};
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

fn run(mut args: Args, conf: &mut Config) -> Res<()> {
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
    let command = args
        .command
        .as_ref()
        .unwrap_or(&Command::Timetable {
            day: None,
            current: false,
        })
        .clone();
    // have a valid user
    let user = if command.user_needed() {
        Usr::load(conf).ok_or("no user found, please create one with `rsfilc user --create`")?
    } else {
        Usr::dummy()
    };

    if !io::stdout().is_terminal() {
        info!("most probably talking to a machine, producing machine output (eg json)");
        eprintln!("welcome to the machine!");
        args.machine = true;
    }
    match command {
        Command::Completions { shell: sh } => {
            info!("creating shell completions for {sh}");
            clap_complete::generate(sh, &mut Args::command(), "rsfilc", &mut io::stdout());
            return Ok(());
        }
        Command::Tui {} => {
            warn!("TUI is not yet written");
            todo!("TUI is to be written (soon)");
        }
        Command::Timetable { day, current } => {
            timetable::handle(day, &user, current, args.machine)?;
        }

        Command::Evals {
            subject: subj,
            filter,
            average,
            ghost,
        } => {
            evals::handle(&user, filter, subj, &ghost, average, &args)?;
        }

        Command::Messages { notes } => {
            messages::handle(notes, &user, &args)?;
        }

        Command::Absences { count, subject } => {
            absences::handle(&user, subject, count, &args)?;
        }

        Command::Tests { subject, past } => {
            announced::handle(past, &user, subject, &args)?;
        }

        Command::User {
            delete: del,
            create,
            switch,
            cache_dir,
            userid,
        } => {
            user::handle(userid, create, conf, del, switch, cache_dir, args.machine)?;
        }

        Command::Schools { search } => {
            schools::handle(search, args.machine)?;
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
    Ok(())
}

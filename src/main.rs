use args::{Args, Command};
use chrono::Local;
use clap::{CommandFactory, Parser};
use config::Config;
use ekreta::Res;
use log::*;
use std::fs::OpenOptions;
use user::Usr;

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
    // have a valid user
    let user = if args.command.user_needed() {
        Usr::load(conf).ok_or("no user found, please create one with `rsfilc user --create`")?
    } else {
        Usr::dummy()
    };

    match args.command {
        Command::Completions { shell: sh } => {
            info!("creating shell completions for {sh}");
            clap_complete::generate(sh, &mut Args::command(), "rsfilc", &mut std::io::stdout());
            return Ok(());
        }
        Command::Tui {} => {
            warn!("TUI is not yet written");
            todo!("TUI is to be written (soon)");
        }
        Command::Timetable {
            day,
            current,
            export_day,
        } => {
            timetable::handle(day.as_ref(), &user, current, export_day)?;
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
            userid,
        } => {
            user::handle(userid, create, conf, delete, switch)?;
        }

        Command::Schools { search } => {
            schools::handle(search)?;
        }
    };
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
/// Fill under `this` with many `with` [`char`]s, inlaying `hint` if any.
///
/// this:   "123456789" <- len: 9
/// with:   '#'
/// hint:   "bab" <- len: 3
///
/// so:     "123456789" <- len: 9
/// result: "12 bab 89" <- len: 9
pub fn fill(this: &str, with: char, hint: Option<&str>) {
    let longest = this.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let inlay_hint = if let Some(il_hint) = hint {
        [" ", il_hint, " "].concat()
    } else {
        String::new()
    };

    let left = (longest - inlay_hint.chars().count()) / 2;
    println!(
        "{}{}{}",
        with.to_string().repeat(left),
        inlay_hint,
        with.to_string()
            .repeat(longest - left - inlay_hint.chars().count())
    );
}

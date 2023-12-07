use crate::args::{AppArgs, parse};
use crate::cli::commands::solve;

mod days;
mod cli;

mod args {
    use crate::days::Day;

    const HELP: &str = "\
🎄 Advent of Code 2023 CLI

USAGE:
  app [OPTIONS]

FLAGS:
  h, help            Prints help information

OPTIONS:
  solve DAY             Run the specified day
";

    pub enum AppArgs {
        Solve {
            day: Day
        }
    }

    pub fn parse() -> Result<AppArgs, anyhow::Error> {
        let mut args = pico_args::Arguments::from_env();
        let app_args = match args.subcommand()?.as_deref() {
            Some("help") | Some("h")=> {
                println!("{HELP}");
                std::process::exit(0);
            }
            Some("solve") => AppArgs::Solve { day: args.free_from_str()? },
            Some(x) => {
                eprintln!("Unknown command: {}, run --help", x);
                std::process::exit(1);
            }
            None => {
                eprintln!("Missing command");
                std::process::exit(1);
            }
        };
        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArgs::Solve { day } => solve::handle(day)
        }
    }
}

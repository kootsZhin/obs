use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use eyre::Result;

use args::*;
use cmd::*;

pub mod args;
pub mod cmd;
pub mod utils;

fn main() -> Result<()> {
    let args = ObsArg::try_parse().unwrap_or_else(|err| {
        // Reference: https://github.com/mgunyho/tere/blob/master/src/main.rs#L28
        // custom error handling: clap writes '--help' and '--version'
        // to stdout by default, but we want to print those to stderr
        // as well to not interfere with the intended behavior of tere
        eprint!("{}", err);
        std::process::exit(1);
    });

    // TODO this part is chunky
    // TODO lots of error introduced after adding flag
    if args.goto.is_some() {
        args_goto(args.goto.unwrap())
    } else if args.open.is_some() {
        args_open(args.open.unwrap())
    } else {
        let obs_opt = ObsOption::to_string_vec();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Hello there! Welcome to obs:")
            .items(&obs_opt)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(index) => match index.try_into() {
                Ok(ObsOption::Goto) => select_goto(),
                Ok(ObsOption::Open) => select_open(),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

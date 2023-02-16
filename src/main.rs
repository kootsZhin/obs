use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use eyre::Result;

use args::*;
use cmd::*;

pub mod args;
pub mod cmd;
pub mod utils;

fn main() -> Result<()> {
    // TODO handle the arguments
    let obs_arg = ObsArg::try_parse().unwrap_or_else(|err| {
        // Reference: https://github.com/mgunyho/tere/blob/master/src/main.rs#L28
        // custom error handling: clap writes '--help' and '--version'
        // to stdout by default, but we want to print those to stderr
        // as well to not interfere with the intended behavior of tere
        eprint!("{}", err);
        std::process::exit(1);
    });

    let obs_opt = ObsOption::to_string_vec();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hello there! Welcome to obs:")
        .items(&obs_opt)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => match index.try_into() {
            Ok(ObsOption::Goto) => goto(),
            Ok(ObsOption::Open) => open(),
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

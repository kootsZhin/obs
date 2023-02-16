use dialoguer::{theme::ColorfulTheme, Select};
use eyre::Result;

pub mod cmd;
pub mod utils;

fn main() -> Result<()> {
    let cmd_options = cmd::CmdOption::to_string_vec();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hello there! Welcome to obs:")
        .items(&cmd_options)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => match index.try_into() {
            Ok(cmd::CmdOption::Goto) => cmd::goto(),
            Ok(cmd::CmdOption::Open) => cmd::open(),
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

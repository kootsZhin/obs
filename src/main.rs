use dialoguer::{theme::ColorfulTheme, Select};

pub mod cmd;

fn main() -> eyre::Result<()> {
    let cmd_options = cmd::CmdOption::to_string_vec();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Hello there! Welcome to obs:")
        .items(&cmd_options)
        .default(0)
        .interact_opt()?;

    // TODO fix the error type
    match selection {
        Some(index) => match index.try_into() {
            Ok(cmd::CmdOption::Goto) => cmd::goto(),
            Ok(cmd::CmdOption::Open) => cmd::open(),
            Err(_) => Ok(()),
        },
        None => Ok(()),
    }
}

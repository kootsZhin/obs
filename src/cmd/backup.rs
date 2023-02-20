use chrono::Local;
use eyre::Result;
use std::process::Command;

use crate::Vault;

pub fn backup() -> Result<()> {
    let is_vault = Vault::is_valid_vault();

    // TODO handle error better
    if !(is_vault) {
        panic!("not in vault")
    }

    let system_time = Local::now();
    let commit_time = system_time.format("%Y-%m-%d %T");

    let commit_message = format!("-m vault backup: {commit_time}", commit_time = commit_time);

    Command::new("git").args(&["add", "."]).output()?;
    Command::new("git")
        .args(&["commit", &commit_message.to_string()])
        .output()?;
    Command::new("git").arg("push").output()?;

    Ok(())
}

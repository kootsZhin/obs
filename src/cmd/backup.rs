use chrono::offset::Utc;
use chrono::DateTime;
use eyre::Result;
use std::process::Command;
use std::time::SystemTime;

pub fn backup() -> Result<()> {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let commit_time = datetime.format("%d-%m-%Y %T");

    let commit_message = format!("vault backup: {commit_time}", commit_time = commit_time);

    Ok(())
}

use super::utils::Vault;
use std::process::Command;

pub fn open() -> eyre::Result<()> {
    let vault_selected = Vault::select_vault().unwrap();

    let obs_url = format!("obsidian://open?vault={vault}", vault = vault_selected);
    Command::new("open")
        .arg(obs_url)
        .spawn()
        .expect("sh command failed to start");

    Ok(())
}

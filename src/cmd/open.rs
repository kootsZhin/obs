use eyre::Result;
use std::process::Command;

use crate::utils::Vault;

pub fn open() -> Result<()> {
    let vault_selected = Vault::select_vault().unwrap();

    let obs_url = format!("obsidian://open?vault={vault}", vault = vault_selected);
    Command::new("open").arg(obs_url).spawn()?;
    Ok(())
}

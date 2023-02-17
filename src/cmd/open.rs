use eyre::Result;
use std::process::Command;

use crate::utils::Vault;

pub fn select_open() -> Result<()> {
    let vault_selected = Vault::select_vault().unwrap();
    _open(vault_selected)
}

pub fn args_open(vault_selected: String) -> Result<()> {
    // TODO add check for vault existence
    _open(vault_selected)
}

fn _open(vault_selected: String) -> Result<()> {
    let obs_url = format!("obsidian://open?vault={vault}", vault = vault_selected);
    Command::new("open").arg(obs_url).spawn()?;
    Ok(())
}

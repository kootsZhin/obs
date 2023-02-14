use super::utils::Vault;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;

pub fn open() -> eyre::Result<()> {
    let vaults = Vault::get_vaults();

    let vault_names: Vec<&String> = vaults
        .iter()
        .map(|Vault { ref name, .. }| name)
        .clone()
        .collect();

    let vault_paths: Vec<&String> = vaults
        .iter()
        .map(|Vault { ref path, .. }| path)
        .clone()
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Open vault:")
        .items(&vault_names)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => {
            let obs_url = format!(
                "obsidian://open?vault={vault}",
                vault = vault_names[index].to_string()
            );
            Command::new("open")
                .arg(obs_url)
                .spawn()
                .expect("sh command failed to start");
        }
        None => eprintln!("Error: No vault is selected, run `obs -h` to see the usage"),
    }

    Ok(())
}

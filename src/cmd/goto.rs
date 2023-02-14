use super::utils::Vault;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn goto() -> eyre::Result<()> {
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
        .with_prompt("Goto vault:")
        .items(&vault_names)
        .default(0)
        .interact_opt()?;

    match selection {
        Some(index) => {
            println!("{}", vault_paths[index]);
        }
        None => eprintln!("Error: No vault is selected, run `obs -h` to see the usage"),
    }

    Ok(())
}

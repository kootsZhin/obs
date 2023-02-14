use super::utils::Vault;
use std::process::Command;

pub fn open() -> eyre::Result<()> {
    let selection = Vault::select_vault().unwrap();

    match selection {
        Some(index) => {
            let obs_url = format!(
                "obsidian://open?vault={vault}",
                vault = Vault::get_vault_name(index)
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

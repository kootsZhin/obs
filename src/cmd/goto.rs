use super::utils::Vault;

pub fn goto() -> eyre::Result<()> {
    let selection = Vault::select_vault().unwrap();

    match selection {
        Some(index) => {
            println!("{}", Vault::get_vault_path(index));
        }
        None => eprintln!("Error: No vault is selected, run `obs -h` to see the usage"),
    }

    Ok(())
}

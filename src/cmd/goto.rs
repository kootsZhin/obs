use super::utils::Vault;

pub fn goto() -> eyre::Result<()> {
    let vault_selected = Vault::select_vault().unwrap();
    println!("{}", Vault::get_vault_path(&vault_selected).unwrap());
    Ok(())
}

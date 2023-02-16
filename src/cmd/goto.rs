use eyre::Result;

use crate::utils::Vault;

pub fn goto() -> Result<()> {
    let vault_selected = Vault::select_vault().unwrap();
    println!("{}", Vault::get_vault_path(&vault_selected));
    Ok(())
}

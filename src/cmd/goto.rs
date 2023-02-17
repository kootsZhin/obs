use eyre::Result;

use crate::utils::Vault;

pub fn select_goto() -> Result<()> {
    let vault_selected = Vault::select_vault().unwrap();
    _goto(vault_selected)
}

pub fn args_goto(vault_selected: String) -> Result<()> {
    _goto(vault_selected)
}

fn _goto(vault_selected: String) -> Result<()> {
    println!("{}", Vault::get_vault_path(&vault_selected));
    Ok(())
}

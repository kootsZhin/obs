use dialoguer::{theme::ColorfulTheme, Select};
use eyre::Result;
use home::home_dir;
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ObsJsonVault {
    open: Option<bool>,
    path: String,
    ts: usize,
}

pub struct Vault {
    pub name: String,
    pub path: String,
}

impl Vault {
    pub fn get_vaults() -> Result<Vec<Vault>> {
        let mut res: Vec<Vault> = Vec::new();

        let path = home_dir().unwrap();
        let cfg_path = format!(
            "{path}/Library/Application Support/obsidian/obsidian.json",
            path = path.to_string_lossy()
        );

        // TODO only working with mac default for now
        let data = read_to_string(cfg_path).unwrap();

        let json: HashMap<String, HashMap<String, ObsJsonVault>> = from_str(&data).unwrap();

        for (_, value) in json.get("vaults").unwrap() {
            let path = value.path.clone();
            let (_, name) = value.path.rsplit_once('/').unwrap();
            res.push(Vault {
                name: name.to_string(),
                path: path,
            })
        }

        Ok(res)
    }

    pub fn get_vault_names() -> Vec<String> {
        let vaults = Vault::get_vaults().unwrap();
        let vault_names: Vec<String> = vaults
            .iter()
            .map(|Vault { ref name, .. }| String::from(name))
            .clone()
            .collect();

        vault_names
    }

    pub fn get_vault_path(vault_name: &String) -> String {
        let vaults = Vault::get_vaults().unwrap();
        let vault_selected: Vec<Vault> = vaults
            .into_iter()
            .filter(|x| x.name == vault_name.to_string())
            .collect();

        vault_selected[0].path.clone()
    }

    pub fn select_vault() -> Result<String> {
        let vault_names = Vault::get_vault_names();
        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Open vault:")
            .items(&vault_names)
            .default(0)
            .interact_opt()?;

        let res = vault_names[selection.unwrap()].clone();

        Ok(res)
    }
}

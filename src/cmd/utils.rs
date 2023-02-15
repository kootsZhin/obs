use dialoguer::{theme::ColorfulTheme, Select};
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

// TODO this part is a bit chunky
// TODO arrage the vaults by ts
impl Vault {
    pub fn get_vaults() -> Vec<Vault> {
        let mut res: Vec<Vault> = Vec::new();
        let cfg_path = match home_dir() {
            Some(path) => format!(
                "{path}/Library/Application Support/obsidian/obsidian.json",
                path = path.to_string_lossy()
            ),
            None => panic!("Impossible to get your home dir!"),
        };

        // TODO default obsidian config folder for mac, need to
        // get os type to change file path (less urgent)
        let data = read_to_string(cfg_path).expect("Unable to read file");

        let json: HashMap<String, HashMap<String, ObsJsonVault>> =
            from_str(&data).expect("JSON was not well-formatted");

        for (_, value) in json.get("vaults").unwrap() {
            let path = value.path.clone();
            let (_, name) = value.path.rsplit_once('/').unwrap();
            res.push(Vault {
                name: name.to_string(),
                path: path,
            })
        }
        res
    }

    pub fn get_vault_names() -> Vec<String> {
        let vaults = Vault::get_vaults();
        let vault_names: Vec<String> = vaults
            .iter()
            .map(|Vault { ref name, .. }| String::from(name))
            .clone()
            .collect();

        vault_names
    }

    pub fn get_vault_path(vault_name: &String) -> eyre::Result<String> {
        let vaults = Vault::get_vaults();
        let vault_selected: Vec<Vault> = vaults
            .into_iter()
            .filter(|x| x.name == vault_name.to_string())
            .collect();
        Ok(vault_selected[0].path.clone())
    }

    pub fn select_vault() -> eyre::Result<String> {
        let vault_names = Vault::get_vault_names();

        let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Open vault:")
            .items(&vault_names)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(index) => Ok(vault_names[index].clone()),
            None => panic!("Error when selecting vault"),
        }
    }
}

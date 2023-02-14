use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct ObsJsonVault {
    open: bool,
    path: String,
    ts: usize,
}

pub struct Vault {
    pub name: String,
    pub path: String,
}

impl Vault {
    pub fn get_vaults() -> Vec<Vault> {
        let mut res: Vec<Vault> = Vec::new();

        // TODO default obsidian config folder for mac, need to
        // (1) get user name to support arbitray mac user
        // (2) get os type to change file path (less urgent)
        let data = read_to_string("/Users/k/Library/Application Support/obsidian/obsidian.json")
            .expect("Unable to read file");

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
}

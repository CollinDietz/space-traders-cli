use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub account_token: String,
    pub agents: Vec<Agent>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Config {
                account_token: String::new(),
                agents: Vec::new(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}

fn config_path() -> PathBuf {
    ProjectDirs::from("com", "CollinDietz", "space-traders-cli")
        .expect("No valid home directory found")
        .config_dir()
        .join("config.json")
}

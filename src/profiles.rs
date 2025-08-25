use dirs;
use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub interval: Option<u64>,
    pub mode: Mode,
    pub button: MouseButton,
    pub repeat: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Mode {
    Click,
    Hold,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub profile: Vec<Profile>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = dirs::config_dir()
            .unwrap_or_default()
            .join("lazyclicker/profiles.toml");

        let contents = fs::read_to_string(config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    // create the main folder
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        let config_path = dirs::config_dir().unwrap_or_default().join("lazyclicker");

        if config_path.exists() {
            return Ok(config_path);
        }

        fs::create_dir_all(&config_path)?;
        Ok(config_path)
    }
}

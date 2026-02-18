use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

const CONFIG_PATH: &str = "data/config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub sender: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config not found. Run: fvl config set-sender <address>")]
    NotFound,

    #[error("Failed to read config: {0}")]
    ReadError(String),

    #[error("Failed to write config: {0}")]
    WriteError(String),

    #[error("Failed to parse config: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl CliConfig {
    /// Load config from data/config.json
    pub fn load() -> Result<Self, ConfigError> {
        if !Path::new(CONFIG_PATH).exists() {
            return Err(ConfigError::NotFound);
        }

        let content = fs::read_to_string(CONFIG_PATH)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;

        let config: CliConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save config to data/config.json
    pub fn save(&self) -> Result<(), ConfigError> {
        if let Some(parent) = Path::new(CONFIG_PATH).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;

        fs::write(CONFIG_PATH, json)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;

        Ok(())
    }

    /// Load or return default Anvil sender
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_else(|_| CliConfig {
            sender: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
        })
    }
}
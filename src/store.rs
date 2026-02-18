use std::fs;
use std::path::Path;
use thiserror::Error;
use crate::state::State;

pub const STATE_PATH: &str = "data/state.json";

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Failed to read state file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse state file: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Failed to write state file: {0}")]
    WriteError(String),
}

pub struct Store;

impl Store {
 
    pub fn load() -> Result<State, StoreError> {
        Self::load_from(STATE_PATH)
    }

    pub fn save(state: &State) -> Result<(), StoreError> {
        Self::save_to(state, STATE_PATH)
    }
    
    pub fn load_from(path: &str) -> Result<State, StoreError> {
        if !Path::new(path).exists() {
            let empty = State::new();
            Self::save_to(&empty, path)?;
            return Ok(empty);
        }

        let content = fs::read_to_string(path)?;

        if content.trim().is_empty() {
            let empty = State::new();
            Self::save_to(&empty, path)?;
            return Ok(empty);
        }

        let state: State = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn save_to(state: &State, path: &str) -> Result<(), StoreError> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| StoreError::WriteError(e.to_string()))?;
        }

        let json = serde_json::to_string_pretty(state)
            .map_err(|e| StoreError::WriteError(e.to_string()))?;

        fs::write(path, json)
            .map_err(|e| StoreError::WriteError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use std::fs;

    fn test_state_path(test_name: &str) -> String {
        format!("data/test_{}/state.json", test_name)
    }

    fn cleanup(test_name: &str) {
        let _ = fs::remove_dir_all(format!("data/test_{}", test_name));
    }

    #[test]
    fn test_save_and_load() {
        cleanup("save_and_load");
        let path = test_state_path("save_and_load");

        let mut state = State::new();
        state.set_balance(
            "0x1234567890123456789012345678901234567890",
            &crate::types::AssetType::Eth,
            9999,
        );

        Store::save_to(&state, &path).unwrap();
        let loaded = Store::load_from(&path).unwrap();

        assert_eq!(
            loaded.get_balance(
                "0x1234567890123456789012345678901234567890",
                &crate::types::AssetType::Eth,
            ),
            9999
        );

        cleanup("save_and_load");
    }
}
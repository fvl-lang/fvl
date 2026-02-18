use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const REGISTRY_PATH: &str = "data/systems.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRegistry {
    /// Maps system_id (hex) → original YAML
    systems: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Failed to read registry: {0}")]
    ReadError(String),

    #[error("Failed to write registry: {0}")]
    WriteError(String),

    #[error("Failed to parse registry: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("System not found in registry: {0}")]
    SystemNotFound(String),
}

impl SystemRegistry {
    /// Create new empty registry
    pub fn new() -> Self {
        SystemRegistry {
            systems: HashMap::new(),
        }
    }

    /// Load registry from disk, create if doesn't exist
    pub fn load() -> Result<Self, RegistryError> {
        if !Path::new(REGISTRY_PATH).exists() {
            let empty = Self::new();
            empty.save()?;
            return Ok(empty);
        }

        let content = fs::read_to_string(REGISTRY_PATH)
            .map_err(|e| RegistryError::ReadError(e.to_string()))?;

        if content.trim().is_empty() {
            let empty = Self::new();
            empty.save()?;
            return Ok(empty);
        }

        let registry: SystemRegistry = serde_json::from_str(&content)?;
        Ok(registry)
    }

    /// Save registry to disk
    pub fn save(&self) -> Result<(), RegistryError> {
        if let Some(parent) = Path::new(REGISTRY_PATH).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| RegistryError::WriteError(e.to_string()))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| RegistryError::WriteError(e.to_string()))?;

        fs::write(REGISTRY_PATH, json)
            .map_err(|e| RegistryError::WriteError(e.to_string()))?;

        Ok(())
    }

    /// Register a new system
    pub fn register(&mut self, system_id: &str, yaml: &str) -> Result<(), RegistryError> {
        self.systems.insert(system_id.to_string(), yaml.to_string());
        self.save()
    }

    /// Retrieve YAML for a system
    pub fn get(&self, system_id: &str) -> Option<&String> {
        self.systems.get(system_id)
    }

    /// Check if system exists
    pub fn contains(&self, system_id: &str) -> bool {
        self.systems.contains_key(system_id)
    }
}

impl Default for SystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_retrieve() {
        let mut registry = SystemRegistry::new();
        
        registry.register(
            "0xabc123",
            "system: Test\npool:\n  collect:\n    from:\n      type: anyone"
        ).unwrap();

        assert!(registry.contains("0xabc123"));
        assert!(registry.get("0xabc123").is_some());
    }
}
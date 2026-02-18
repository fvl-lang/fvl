use crate::types::FvlSystem;
use crate::validation::Validator;
use serde_yaml;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid YAML syntax: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub struct Parser;

impl Parser {
    pub fn parse_file(path: &str) -> Result<FvlSystem, ParseError> {
        let content = fs::read_to_string(path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ParseError::FileNotFound(path.to_string())
                } else {
                    ParseError::IoError(e)
                }
            })?;
        
        Self::parse_yaml(&content)
    }
    
    pub fn parse_yaml(yaml: &str) -> Result<FvlSystem, ParseError> {
        let system: FvlSystem = serde_yaml::from_str(yaml)
            .map_err(ParseError::YamlError)?;
        
        Validator::validate(&system)?;
        
        Ok(system)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_system() {
        let yaml = r#"
system: "SimpleSwap"

pool:
  collect:
    from:
      type: anyone
    what:
      type: eth
    min:
      type: zero
    max:
      type: none
    cap:
      type: none

rules:
  conditions:
    - if:
        type: swap_requested
      then:
        type: execute_swap
  
  distribute:
    formula:
      type: proportional
    to:
      type: contributors
    triggers: on_swap

rights:
  anyone: [swap, add_liquidity]

time:
  start:
    type: now
  end:
    type: none
  locks:
    type: none
  vesting:
    type: none

oracles: []
"#;

        let result = Parser::parse_yaml(yaml);
        assert!(result.is_ok());
        
        let system = result.unwrap();
        assert_eq!(system.system, "SimpleSwap");
    }

    #[test]
    fn test_parse_missing_field() {
        let yaml = r#"
system: "Test"
pool:
  collect:
    from:
      type: anyone
"#;

        let result = Parser::parse_yaml(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = r#"
system: "Test"
invalid yaml: [
"#;

        let result = Parser::parse_yaml(yaml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::YamlError(_)));
    }

    #[test]
    fn test_parse_file_not_found() {
        let result = Parser::parse_file("nonexistent.yaml");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::FileNotFound(_)));
    }
}
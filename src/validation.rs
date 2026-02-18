use crate::types::{FvlSystem, AssetType, AccessRule};
use crate::parser::ParseError;
use regex::Regex;

pub struct Validator;

impl Validator {
    pub fn validate(system: &FvlSystem) -> Result<(), ParseError> {
        Self::validate_system_name(&system.system)?;
        Self::validate_asset_addresses(&system.pool.collect.what)?;
        Self::validate_access_rule_addresses(&system.pool.collect.from)?;
        Self::validate_oracle_references(system)?;
        
        Ok(())
    }
    
    fn validate_system_name(name: &str) -> Result<(), ParseError> {
        if name.is_empty() {
            return Err(ParseError::ValidationError(
                "System name cannot be empty".to_string()
            ));
        }
        
        if name.len() > 64 {
            return Err(ParseError::ValidationError(
                format!("System name too long: {} (max 64 characters)", name.len())
            ));
        }
        
        Ok(())
    }
    
    fn validate_ethereum_address(address: &str) -> Result<(), ParseError> {
        let re = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
        
        if !re.is_match(address) {
            return Err(ParseError::ValidationError(
                format!("Invalid Ethereum address format: {}", address)
            ));
        }
        
        Ok(())
    }
    
    fn validate_asset_addresses(asset: &AssetType) -> Result<(), ParseError> {
        match asset {
            AssetType::Eth => Ok(()),
            AssetType::Erc20 { address } => Self::validate_ethereum_address(address),
            AssetType::Erc721 { address } => Self::validate_ethereum_address(address),
            AssetType::Erc1155 { address, .. } => Self::validate_ethereum_address(address),
            AssetType::Multiple { assets } => {
                for a in assets {
                    Self::validate_asset_addresses(a)?;
                }
                Ok(())
            }
        }
    }
    
    fn validate_access_rule_addresses(rule: &AccessRule) -> Result<(), ParseError> {
        match rule {
            AccessRule::Anyone => Ok(()),
            AccessRule::TokenHolders { address } => Self::validate_ethereum_address(address),
            AccessRule::NftHolders { address } => Self::validate_ethereum_address(address),
            AccessRule::Whitelist { addresses } => {
                for addr in addresses {
                    Self::validate_ethereum_address(addr)?;
                }
                Ok(())
            }
            AccessRule::MinBalance { token, .. } => Self::validate_ethereum_address(token),
        }
    }
    
    fn validate_oracle_references(system: &FvlSystem) -> Result<(), ParseError> {
        let oracle_names: std::collections::HashSet<_> = 
            system.oracles.iter().map(|o| o.name.as_str()).collect();
        
        for condition in &system.rules.conditions {
            match &condition.if_expr {
                crate::types::Expression::PriceLt { oracle, .. } |
                crate::types::Expression::PriceGt { oracle, .. } |
                crate::types::Expression::PriceEq { oracle, .. } => {
                    if !oracle_names.contains(oracle.as_str()) {
                        return Err(ParseError::ValidationError(
                            format!("Oracle '{}' referenced but not defined", oracle)
                        ));
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use std::collections::HashMap;

    fn create_minimal_system(name: &str) -> FvlSystem {
        FvlSystem {
            system: name.to_string(),
            pool: Pool {
                collect: Collect {
                    from: AccessRule::Anyone,
                    what: AssetType::Eth,
                    min: Amount::Zero,
                    max: MaxAmount::None,
                    cap: CapAmount::None,
                },
            },
            rules: Rules {
                conditions: vec![],
                distribute: Distribute {
                    formula: DistributionType::Proportional,
                    to: RecipientGroup::Contributors,
                    triggers: Trigger::Manual,
                },
            },
            rights: HashMap::new(),
            time: Time {
                start: TimeValue::Now,
                end: TimeValue::None,
                locks: LockValue::None,
                vesting: VestingValue::None,
                cliffs: None,
            },
            oracles: vec![],
        }
    }

    #[test]
    fn test_validate_empty_system_name() {
        let mut system = create_minimal_system("");
        system.system = "".to_string();
        
        let result = Validator::validate(&system);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_address() {
        let result = Validator::validate_ethereum_address("invalid");
        assert!(result.is_err());
        
        let result = Validator::validate_ethereum_address("0x123");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_address() {
        let result = Validator::validate_ethereum_address(
            "0x1234567890123456789012345678901234567890"
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_undefined_oracle() {
        let mut system = create_minimal_system("Test");
        system.rules.conditions.push(Condition {
            if_expr: Expression::PriceGt {
                oracle: "undefined_oracle".to_string(),
                value: 100,
            },
            then: Action::Pause,
        });
        
        let result = Validator::validate(&system);
        assert!(result.is_err());
    }
}
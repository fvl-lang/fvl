use crate::types::FvlSystem;
use sha3::{Keccak256, Digest};
//use serde_yaml;

pub type SystemId = [u8; 32];

pub fn compute_system_id(system: &FvlSystem) -> SystemId {
    let canonical_json = serde_json::to_string(system)
        .expect("Failed to serialize system to JSON");
    keccak256(canonical_json.as_bytes())
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

pub fn system_id_to_hex(id: &SystemId) -> String {
    format!("0x{}", hex::encode(id))
}

pub fn system_id_from_hex(hex_str: &str) -> Result<SystemId, hex::FromHexError> {
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(hex_str)?;

    if bytes.len() != 32 {
        return Err(hex::FromHexError::InvalidStringLength);
    }

    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes);
    Ok(id)
}

// fn to_canonical_yaml(system: &FvlSystem) -> String {
//     serde_yaml::to_string(system)
//         .expect("Failed to serialize system to YAML")
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use std::collections::HashMap;

    fn minimal_system() -> FvlSystem {
        FvlSystem {
            system: "Test".to_string(),
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
    fn test_deterministic_hashing() {
        let system = minimal_system();
        let id1 = compute_system_id(&system);
        let id2 = compute_system_id(&system);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_systems_different_hashes() {
        let system1 = minimal_system();
        let mut system2 = system1.clone();
        system2.system = "Test2".to_string();

        let id1 = compute_system_id(&system1);
        let id2 = compute_system_id(&system2);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_hex_prefix() {
        let system = minimal_system();
        let id = compute_system_id(&system);
        let hex = system_id_to_hex(&id);
        assert!(hex.starts_with("0x"));
    }

    #[test]
    fn test_hex_roundtrip() {
        let system = minimal_system();
        let id = compute_system_id(&system);
        let hex = system_id_to_hex(&id);
        let parsed = system_id_from_hex(&hex).unwrap();
        assert_eq!(id, parsed);
    }
}
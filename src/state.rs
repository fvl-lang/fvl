use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::hash::{SystemId, keccak256, system_id_to_hex};
use crate::types::{FvlSystem, AssetType};

pub type Address = [u8; 20];

pub type OracleValue = u128;

pub type OracleName = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BalanceKey {
    pub address: String,
    pub asset_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OracleKey {
    pub system_id: String,
    pub oracle_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetadata {
    pub deployed_at: u64,
    pub deployer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub system_id: String,
    #[serde(flatten)]
    pub system: FvlSystem,
    pub metadata: SystemMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub systems: HashMap<String, SystemState>,

    #[serde(with = "balance_map")]
    pub balances: HashMap<BalanceKey, u128>,

    #[serde(with = "oracle_map")]
    pub oracles: HashMap<OracleKey, OracleValue>,

    pub nonces: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        State {
            systems: HashMap::new(),
            balances: HashMap::new(),
            oracles: HashMap::new(),
            nonces: HashMap::new(),
        }
    }

    pub fn compute_state_root(&self) -> [u8; 32] {
        let serialized = self.to_canonical_bytes();
        keccak256(&serialized)
    }

    pub fn state_root_hex(&self) -> String {
        let root = self.compute_state_root();
        format!("0x{}", hex::encode(root))
    }

    pub fn deploy_system(
        &mut self,
        system: FvlSystem,
        system_id: SystemId,
        deployer: String,
        timestamp: u64,
    ) -> Result<(), StateError> {
        let system_id_hex = system_id_to_hex(&system_id);

        if self.systems.contains_key(&system_id_hex) {
            return Err(StateError::SystemAlreadyDeployed(system_id_hex));
        }

        for oracle in &system.oracles {
            let key = OracleKey {
                system_id: system_id_hex.clone(),
                oracle_name: oracle.name.clone(),
            };
            self.oracles.insert(key, 0);
        }

        let system_state = SystemState {
            system_id: system_id_hex.clone(),
            system,
            metadata: SystemMetadata {
                deployed_at: timestamp,
                deployer,
            },
        };

        self.systems.insert(system_id_hex, system_state);
        Ok(())
    }

    pub fn get_balance(&self, address: &str, asset: &AssetType) -> u128 {
        let key = BalanceKey {
            address: address.to_string(),
            asset_id: asset_to_id(asset),
        };
        *self.balances.get(&key).unwrap_or(&0)
    }

    pub fn set_balance(&mut self, address: &str, asset: &AssetType, amount: u128) {
        let key = BalanceKey {
            address: address.to_string(),
            asset_id: asset_to_id(asset),
        };
        self.balances.insert(key, amount);
    }

    pub fn get_oracle(&self, system_id: &str, oracle_name: &str) -> Option<OracleValue> {
        let key = OracleKey {
            system_id: system_id.to_string(),
            oracle_name: oracle_name.to_string(),
        };
        self.oracles.get(&key).copied()
    }

    pub fn set_oracle(
        &mut self,
        system_id: &str,
        oracle_name: &str,
        value: OracleValue,
    ) -> Result<(), StateError> {
        let key = OracleKey {
            system_id: system_id.to_string(),
            oracle_name: oracle_name.to_string(),
        };

        if !self.oracles.contains_key(&key) {
            return Err(StateError::OracleNotFound(oracle_name.to_string()));
        }

        self.oracles.insert(key, value);
        Ok(())
    }

    pub fn get_nonce(&self, address: &str) -> u64 {
        *self.nonces.get(address).unwrap_or(&0)
    }

    pub fn consume_nonce(
        &mut self,
        address: &str,
        submitted_nonce: u64,
    ) -> Result<(), StateError> {
        let current = self.get_nonce(address);

        if submitted_nonce != current {
            return Err(StateError::InvalidNonce {
                address: address.to_string(),
                expected: current,
                got: submitted_nonce,
            });
        }

        self.nonces.insert(address.to_string(), current + 1);
        Ok(())
    }

    fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut systems: Vec<_> = self.systems.iter().collect();
        systems.sort_by_key(|(k, _)| k.as_str());

        let mut balances: Vec<_> = self.balances.iter().collect();
        balances.sort_by(|(a, _), (b, _)| {
            a.address.cmp(&b.address)
                .then(a.asset_id.cmp(&b.asset_id))
        });

        let mut oracles: Vec<_> = self.oracles.iter().collect();
        oracles.sort_by(|(a, _), (b, _)| {
            a.system_id.cmp(&b.system_id)
                .then(a.oracle_name.cmp(&b.oracle_name))
        });

        let mut nonces: Vec<_> = self.nonces.iter().collect();
        nonces.sort_by_key(|(k, _)| k.as_str());

        #[derive(Serialize)]
        struct CanonicalState<'a> {
            systems: Vec<(&'a String, &'a SystemState)>,
            balances: Vec<(&'a BalanceKey, &'a u128)>,
            oracles: Vec<(&'a OracleKey, &'a OracleValue)>,
            nonces: Vec<(&'a String, &'a u64)>,
        }

        let canonical = CanonicalState {
            systems,
            balances,
            oracles,
            nonces,
        };

        serde_json::to_vec(&canonical)
            .expect("Failed to serialize state")
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

fn asset_to_id(asset: &AssetType) -> String {
    match asset {
        AssetType::Eth => "ETH".to_string(),
        AssetType::Erc20 { address } => format!("ERC20:{}", address),
        AssetType::Erc721 { address } => format!("ERC721:{}", address),
        AssetType::Erc1155 { address, id } => format!("ERC1155:{}:{}", address, id),
        AssetType::Multiple { .. } => "MULTIPLE".to_string(),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("System already deployed: {0}")]
    SystemAlreadyDeployed(String),

    #[error("System not found: {0}")]
    SystemNotFound(String),

    #[error("Oracle not found: {0}")]
    OracleNotFound(String),

    #[error("Insufficient balance for address {address}: required {required}, has {available}")]
    InsufficientBalance {
        address: String,
        required: u128,
        available: u128,
    },

    #[error("Invalid nonce for {address}: expected {expected}, got {got}")]
    InvalidNonce {
        address: String,
        expected: u64,
        got: u64,
    },

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

mod balance_map {
    use super::{BalanceKey, HashMap};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        map: &HashMap<BalanceKey, u128>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<(&BalanceKey, &u128)> = map.iter().collect();
        entries.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<BalanceKey, u128>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries: Vec<(BalanceKey, u128)> = Vec::deserialize(deserializer)?;
        Ok(entries.into_iter().collect())
    }
}

mod oracle_map {
    use super::{OracleKey, OracleValue, HashMap};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        map: &HashMap<OracleKey, OracleValue>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<(&OracleKey, &OracleValue)> = map.iter().collect();
        entries.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<OracleKey, OracleValue>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries: Vec<(OracleKey, OracleValue)> = Vec::deserialize(deserializer)?;
        Ok(entries.into_iter().collect())
    }
}

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
    fn test_new_state_is_empty() {
        let state = State::new();
        assert!(state.systems.is_empty());
        assert!(state.balances.is_empty());
        assert!(state.oracles.is_empty());
    }

    #[test]
    fn test_deploy_system() {
        let mut state = State::new();
        let system = minimal_system();
        let system_id = [0u8; 32];

        let result = state.deploy_system(
            system,
            system_id,
            "0x1234567890123456789012345678901234567890".to_string(),
            1000000,
        );

        assert!(result.is_ok());
        assert_eq!(state.systems.len(), 1);
    }

    #[test]
    fn test_deploy_duplicate_system() {
        let mut state = State::new();
        let system = minimal_system();
        let system_id = [0u8; 32];

        state.deploy_system(
            system.clone(),
            system_id,
            "0x1234567890123456789012345678901234567890".to_string(),
            1000000,
        ).unwrap();

        let result = state.deploy_system(
            system,
            system_id,
            "0x1234567890123456789012345678901234567890".to_string(),
            1000000,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_balance_operations() {
        let mut state = State::new();
        let asset = AssetType::Eth;
        let address = "0x1234567890123456789012345678901234567890";

        assert_eq!(state.get_balance(address, &asset), 0);

        state.set_balance(address, &asset, 1000);
        assert_eq!(state.get_balance(address, &asset), 1000);
    }

    #[test]
    fn test_state_root_determinism() {
        let state = State::new();
        let root1 = state.compute_state_root();
        let root2 = state.compute_state_root();
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_state_root_changes_on_update() {
        let mut state = State::new();
        let root1 = state.compute_state_root();

        state.set_balance(
            "0x1234567890123456789012345678901234567890",
            &AssetType::Eth,
            1000,
        );

        let root2 = state.compute_state_root();
        assert_ne!(root1, root2);
    }
}
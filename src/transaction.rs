use serde::{Deserialize, Serialize};
use crate::types::u128_as_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub nonce: u64,
    pub payload: TransactionPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TransactionPayload {
    DeploySystem {
        system_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        yaml: Option<String>,
    },
    Interact {
        system_id: String,
        mode: InteractMode,
    },
    OracleUpdate {
        system_id: String,
        oracle_name: String,
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    Transfer {
        from: String,
        to: String,
        asset_type: TransactionAsset,
        #[serde(with = "u128_as_string")]
        amount: u128,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InteractMode {
    TriggerAction { action: String },
    EvaluateConditions,
    Both { action: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TransactionAsset {
    Eth,
    Erc20 { address: String },
    Erc721 { address: String },
    Erc1155 {
        address: String,
        #[serde(with = "u128_as_string")]
        id: u128,
    },
}
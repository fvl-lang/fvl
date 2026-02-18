use crate::block::Block;
use crate::log::{BlockLog, LogError, LOG_PATH};
use crate::runtime::{Runtime, TxResult};
use crate::state::State;
use crate::store::{Store, StoreError, STATE_PATH};
use crate::transaction::{Transaction, TransactionPayload};
//use crate::parser::Parser;
//use crate::hash::{compute_system_id, system_id_to_hex};
use crate::system_registry::SystemRegistry;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SequencerError {
    #[error("Transaction failed: {0}")]
    TxFailed(String),

    #[error("Log error: {0}")]
    LogError(#[from] LogError),

    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),

    #[error("Registry error: {0}")]
    RegistryError(#[from] crate::system_registry::RegistryError),
}

#[derive(Debug)]
pub struct SequenceResult {
    pub block: Block,
    pub tx_result: TxResult,
}

pub fn sequence_tx(
    tx: Transaction,
    state: &State,
) -> Result<(SequenceResult, State), SequencerError> {
    sequence_tx_at(tx, state, LOG_PATH, STATE_PATH)
}

pub fn sequence_tx_at(
    tx: Transaction,
    state: &State,
    log_path: &str,
    state_path: &str,
) -> Result<(SequenceResult, State), SequencerError> {
    let latest = BlockLog::latest_from(log_path)?;

    let (prev_hash, next_number) = match latest {
        Some(block) => (block.hash.clone(), block.number + 1),
        None => {
            let genesis = BlockLog::init_if_empty_at(log_path, "FVL_TESTNET")?;
            (genesis.hash, 1)
        }
    };

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_tx = if let TransactionPayload::DeploySystem { system_id, .. } = &tx.payload {
        Transaction {
            sender: tx.sender.clone(),
            nonce: tx.nonce,
            payload: TransactionPayload::DeploySystem {
                system_id: system_id.clone(),
                yaml: None, 
            },
        }
    } else {
        tx.clone()
    };

    let (new_state, tx_result) = Runtime::apply_tx(state, tx.clone());
    if tx_result.success {
        if let TransactionPayload::DeploySystem { system_id, yaml } = &tx.payload {
            if let Some(yaml_content) = yaml {
                if !yaml_content.is_empty() {
                    let mut registry = SystemRegistry::load()?;
                    registry.register(system_id, yaml_content)?;
                }
            }
        }
    }

    let state_root = new_state.state_root_hex();
    let block = Block::new_with_timestamp(next_number, prev_hash, vec![log_tx], state_root, timestamp);
    
    BlockLog::append_to(&block, log_path)?;
    Store::save_to(&new_state, state_path)?;

    Ok((SequenceResult { block, tx_result }, new_state))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::BlockLog;
    use crate::state::State;
    use crate::store::Store;
    use crate::transaction::{Transaction, TransactionPayload, TransactionAsset};

    const SENDER: &str = "0x1234567890123456789012345678901234567890";
    const RECEIVER: &str = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd";

 fn test_paths(test_name: &str) -> (String, String) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (
        format!("data/test_{}_{}/blocks.log", test_name, unique),
        format!("data/test_{}_{}/state.json", test_name, unique),
    )
}

  fn cleanup(test_name: &str) {
    if let Ok(entries) = std::fs::read_dir("data") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(&format!("test_{}", test_name)) {
                let _ = std::fs::remove_dir_all(entry.path());
            }
        }
    }
}

    fn transfer_tx(nonce: u64, amount: u128) -> Transaction {
        Transaction {
            sender: SENDER.to_string(),
            nonce,
            payload: TransactionPayload::Transfer {
                from: SENDER.to_string(),
                to: RECEIVER.to_string(),
                asset_type: TransactionAsset::Eth,
                amount,
            },
        }
    }

    fn sequence_tx_isolated(
        tx: Transaction,
        state: &State,
        log_path: &str,
        state_path: &str,
    ) -> Result<(SequenceResult, State), SequencerError> {
        sequence_tx_at(tx, state, log_path, state_path)
    }

    #[test]
    fn test_sequence_single_tx() {
        cleanup("seq_single");
        let (log_path, state_path) = test_paths("seq_single");

        let mut state = State::new();
        state.set_balance(SENDER, &crate::types::AssetType::Eth, 1000);

        let tx = transfer_tx(0, 100);
        let (result, new_state) = sequence_tx_isolated(tx, &state, &log_path, &state_path).unwrap();

        assert!(result.tx_result.success);
        assert_eq!(result.block.number, 1);
        assert_eq!(new_state.get_balance(SENDER, &crate::types::AssetType::Eth), 900);
        assert_eq!(new_state.get_balance(RECEIVER, &crate::types::AssetType::Eth), 100);

        //cleanup("seq_single"); // COMMENT OUT FOR CLEANUP AFTER TEST RUNS
    }

    #[test]
    fn test_sequence_multiple_txs_chain() {
        cleanup("seq_multi");
        let (log_path, state_path) = test_paths("seq_multi");

        let mut state = State::new();
        state.set_balance(SENDER, &crate::types::AssetType::Eth, 1000);

        let tx1 = transfer_tx(0, 100);
        let (result1, state) = sequence_tx_isolated(tx1, &state, &log_path, &state_path).unwrap();
        assert_eq!(result1.block.number, 1);

        let tx2 = transfer_tx(1, 200);
        let (result2, state) = sequence_tx_isolated(tx2, &state, &log_path, &state_path).unwrap();
        assert_eq!(result2.block.number, 2);

        assert_eq!(result2.block.prev_hash, result1.block.hash);
        assert_eq!(state.get_balance(SENDER, &crate::types::AssetType::Eth), 700);
        assert_eq!(state.get_balance(RECEIVER, &crate::types::AssetType::Eth), 300);

        //cleanup("seq_multi"); // COMMENT OUT FOR CLEANUP AFTER TEST RUNS
    }

    #[test]
    fn test_failed_tx_still_produces_block() {
        cleanup("seq_failed");
        let (log_path, state_path) = test_paths("seq_failed");

        let state = State::new();

        let tx = transfer_tx(0, 500);
        let (result, _) = sequence_tx_isolated(tx, &state, &log_path, &state_path).unwrap();

        assert_eq!(result.block.number, 1);
        assert!(!result.tx_result.success);

        let blocks = BlockLog::read_all_from(&log_path).unwrap();
        assert_eq!(blocks.len(), 2); 

        cleanup("seq_failed");
    }

    #[test]
    fn test_state_rebuilt_from_log() {
        cleanup("seq_rebuild");
        let (log_path, state_path) = test_paths("seq_rebuild");

        let mut state = State::new();
        state.set_balance(SENDER, &crate::types::AssetType::Eth, 1000);
        Store::save_to(&state, &state_path).unwrap();

        let tx1 = transfer_tx(0, 100);
        let (_, state) = sequence_tx_isolated(tx1, &state, &log_path, &state_path).unwrap();

        let tx2 = transfer_tx(1, 200);
        let (_, _) = sequence_tx_isolated(tx2, &state, &log_path, &state_path).unwrap();

        let rebuilt = BlockLog::rebuild_state_at(&log_path, &state_path).unwrap();

        assert_eq!(rebuilt.get_balance(SENDER, &crate::types::AssetType::Eth), 700);
        assert_eq!(rebuilt.get_balance(RECEIVER, &crate::types::AssetType::Eth), 300);

        cleanup("seq_rebuild");
    }
}
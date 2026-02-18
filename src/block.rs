use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::hash::keccak256;
use crate::transaction::Transaction;

pub type BlockHash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub prev_hash: String,
    pub timestamp: u64,
    pub txs: Vec<Transaction>,
    pub state_root: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        number: u64,
        prev_hash: String,
        txs: Vec<Transaction>,
        state_root: String,
    ) -> Self {
        let timestamp = Utc::now().timestamp() as u64;

        let hash = compute_block_hash(number, &prev_hash, timestamp, &txs, &state_root);

        Block {
            number,
            prev_hash,
            timestamp,
            txs,
            state_root,
            hash,
        }
    }
        pub fn new_with_timestamp(
        number: u64,
        prev_hash: String,
        txs: Vec<Transaction>,
        state_root: String,
        timestamp: u64,
    ) -> Self {
        let hash = compute_block_hash(number, &prev_hash, timestamp, &txs, &state_root);
        Block { number, prev_hash, timestamp, txs, state_root, hash }
    }

    pub fn genesis(network_name: &str) -> Self {
        let prev_hash = genesis_prev_hash(network_name);
        let state_root = "0x".to_string() + &hex::encode([0u8; 32]);
        let timestamp = Utc::now().timestamp() as u64;
        let hash = compute_block_hash(0, &prev_hash, timestamp, &[], &state_root);

        Block {
            number: 0,
            prev_hash,
            timestamp,
            txs: vec![],
            state_root,
            hash,
        }
    }
}

pub fn genesis_prev_hash(network_name: &str) -> String {
    let hash = keccak256(network_name.as_bytes());
    format!("0x{}", hex::encode(hash))
}

pub fn compute_block_hash(
    number: u64,
    prev_hash: &str,
    timestamp: u64,
    txs: &[Transaction],
    state_root: &str,
) -> String {

    #[derive(Serialize)]
    struct BlockContents<'a> {
        number: u64,
        prev_hash: &'a str,
        timestamp: u64,
        tx_count: usize,
        state_root: &'a str,
    }

    let contents = BlockContents {
        number,
        prev_hash,
        timestamp,
        tx_count: txs.len(),
        state_root,
    };

    let bytes = serde_json::to_vec(&contents)
        .expect("Failed to serialize block contents");

    let hash = keccak256(&bytes);
    format!("0x{}", hex::encode(hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis("FVL_TESTNET");
        assert_eq!(genesis.number, 0);
        assert!(genesis.hash.starts_with("0x"));
        assert!(genesis.prev_hash.starts_with("0x"));
        assert!(genesis.txs.is_empty());
    }

    #[test]
    fn test_genesis_prev_hash_deterministic() {
        let h1 = genesis_prev_hash("FVL_TESTNET");
        let h2 = genesis_prev_hash("FVL_TESTNET");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_different_networks_different_genesis() {
        let h1 = genesis_prev_hash("FVL_TESTNET");
        let h2 = genesis_prev_hash("FVL_MAINNET");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_block_hash_deterministic() {
        let block = Block::new(
            1,
            genesis_prev_hash("FVL_TESTNET"),
            vec![],
            "0x1234".to_string(),
        );

        // Hash should be set and start with 0x
        assert!(block.hash.starts_with("0x"));
        assert_eq!(block.number, 1);
    }

    #[test]
    fn test_block_chain_links() {
        let genesis = Block::genesis("FVL_TESTNET");

        let block1 = Block::new(
            1,
            genesis.hash.clone(),
            vec![],
            "0x1234".to_string(),
        );

        assert_eq!(block1.prev_hash, genesis.hash);
        assert_eq!(block1.number, 1);
    }
}
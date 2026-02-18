use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::log::{BlockLog, LOG_PATH};
use crate::block::Block;

const CONTRACT_PATH: &str = "data/contract.json";
const DEFAULT_SUBMIT_INTERVAL: u64 = 5;
const DEFAULT_POLL_INTERVAL_SECS: u64 = 10;

const LOCAL_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    pub address: String,
    pub deployer: String,
    pub network: String,
    pub rpc_url: String,
}

#[derive(Error, Debug)]
pub enum SubmitterError {
    #[error("Contract config not found at {0}")]
    ConfigNotFound(String),

    #[error("Failed to read contract config: {0}")]
    ConfigReadError(String),

    #[error("Failed to parse contract config: {0}")]
    ConfigParseError(#[from] serde_json::Error),

    #[error("Log error: {0}")]
    LogError(#[from] crate::log::LogError),

    #[error("Cast command failed: {0}")]
    CastError(String),

    #[error("No blocks to submit")]
    NoBlocks,
}

pub struct Submitter {
    pub config: ContractConfig,
    pub submit_interval: u64,
    pub poll_interval_secs: u64,
    pub last_submitted_block: u64,
}

impl Submitter {
    pub fn new() -> Result<Self, SubmitterError> {
        let config = Self::load_contract_config()?;

        let submit_interval = std::env::var("FVL_SUBMIT_INTERVAL")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_SUBMIT_INTERVAL);

        let poll_interval_secs = std::env::var("FVL_POLL_INTERVAL")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);

        println!("Submit interval:  every {} blocks", submit_interval);
        println!("Poll interval:    every {}s", poll_interval_secs);
        println!("Contract:         {}", config.address);
        println!("Network:          {}", config.network);
        println!("RPC:              {}", config.rpc_url);

        Ok(Submitter {
            config,
            submit_interval,
            poll_interval_secs,
            last_submitted_block: 0,
        })
    }

    fn load_contract_config() -> Result<ContractConfig, SubmitterError> {
        if !Path::new(CONTRACT_PATH).exists() {
            return Err(SubmitterError::ConfigNotFound(CONTRACT_PATH.to_string()));
        }

        let content = fs::read_to_string(CONTRACT_PATH)
            .map_err(|e| SubmitterError::ConfigReadError(e.to_string()))?;

        let config: ContractConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn run(&mut self) {
        println!("\nSubmitter started");
        println!("   Watching: {}", LOG_PATH);

        loop {
            match self.poll() {
                Ok(Some(block_number)) => {
                    println!("Submitted state root for block #{}", block_number);
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("Submitter error: {}", e);
                }
            }

            thread::sleep(Duration::from_secs(self.poll_interval_secs));
        }
    }

    pub fn poll(&mut self) -> Result<Option<u64>, SubmitterError> {
        let latest = BlockLog::latest()?;

        let latest_block = match latest {
            Some(b) => b,
            None => return Ok(None),
        };

        if latest_block.number == 0 {
            return Ok(None);
        }

        if latest_block.number <= self.last_submitted_block {
            return Ok(None);
        }

        let blocks_since_last = latest_block.number - self.last_submitted_block;
        if blocks_since_last < self.submit_interval {
            println!(
                "Waiting: {}/{} blocks accumulated since last submission",
                blocks_since_last,
                self.submit_interval
            );
            return Ok(None);
        }

        self.submit_state_root(&latest_block)?;
        self.last_submitted_block = latest_block.number;

        Ok(Some(latest_block.number))
    }

    fn submit_state_root(&self, block: &Block) -> Result<(), SubmitterError> {
        println!(
            "\nSubmitting block #{} state root: {}",
            block.number,
            block.state_root
        );

        let state_root_bytes = self.format_bytes32(&block.state_root)?;

        let output = Command::new("cast")
            .args([
                "send",
                &self.config.address,
                "submitStateRoot(uint256,bytes32)",
                &block.number.to_string(),
                &state_root_bytes,
                "--rpc-url",
                &self.config.rpc_url,
                "--private-key",
                LOCAL_PRIVATE_KEY,
            ])
            .output()
            .map_err(|e| SubmitterError::CastError(
                format!("Failed to run cast: {}", e)
            ))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SubmitterError::CastError(
                format!("cast send failed: {}", stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Transaction hash: {}", Self::extract_tx_hash(&stdout));

        self.verify_submission(block.number, &block.state_root)?;

        Ok(())
    }

    fn verify_submission(
        &self,
        _block_number: u64,
        _expected_root: &str,
    ) -> Result<(), SubmitterError> {
        let output = Command::new("cast")
            .args([
                "call",
                &self.config.address,
                "getLatest()(uint256,bytes32)",
                "--rpc-url",
                &self.config.rpc_url,
            ])
            .output()
            .map_err(|e| SubmitterError::CastError(
                format!("Failed to run cast call: {}", e)
            ))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("On-chain state:    {}", stdout.trim());
        }

        Ok(())
    }

    fn format_bytes32(&self, hex_str: &str) -> Result<String, SubmitterError> {
        let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);

        if stripped.len() > 64 {
            return Err(SubmitterError::CastError(
                format!("State root too long: {}", stripped.len())
            ));
        }

        let padded = format!("0x{:0>64}", stripped);
        Ok(padded)
    }

    fn extract_tx_hash(output: &str) -> &str {
        for line in output.lines() {
            if line.starts_with("transactionHash") {
                return line.split_whitespace().last().unwrap_or("unknown");
            }
        }
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes32_short() {
        let submitter = Submitter {
            config: ContractConfig {
                address: "0x0".to_string(),
                deployer: "0x0".to_string(),
                network: "local".to_string(),
                rpc_url: "http://localhost:8545".to_string(),
            },
            submit_interval: 5,
            poll_interval_secs: 10,
            last_submitted_block: 0,
        };

        let result = submitter.format_bytes32("0xabc").unwrap();
        assert_eq!(result.len(), 66);
        assert!(result.starts_with("0x"));
    }

    #[test]
    fn test_format_bytes32_full() {
        let submitter = Submitter {
            config: ContractConfig {
                address: "0x0".to_string(),
                deployer: "0x0".to_string(),
                network: "local".to_string(),
                rpc_url: "http://localhost:8545".to_string(),
            },
            submit_interval: 5,
            poll_interval_secs: 10,
            last_submitted_block: 0,
        };

        let full_root = "0xdf6f2425d678d5449329048dd175444cf8f051ae4510c758f901c6c2258255da";
        let result = submitter.format_bytes32(full_root).unwrap();
        assert_eq!(result.len(), 66);
        assert!(result.starts_with("0x"));
    }
}

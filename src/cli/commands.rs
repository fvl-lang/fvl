use crate::cli::config::CliConfig;
use crate::cli::output::Output;
use crate::log::BlockLog;
use crate::parser::Parser;
use crate::sequencer::sequence_tx;
use crate::store::Store;
use crate::transaction::{Transaction, TransactionPayload, TransactionAsset, InteractMode};
use crate::hash::{compute_system_id, system_id_to_hex};

/// Deploy a YAML system file
pub fn cmd_deploy(yaml_path: &str, as_json: bool) {
    // Parse and validate
    let system = match Parser::parse_file(yaml_path) {
        Ok(s) => s,
        Err(e) => {
            Output::error(&format!("Parse error: {}", e));
            return;
        }
    };

    let system_id_hex = system_id_to_hex(&compute_system_id(&system));

    if !as_json {
        Output::header("Deploying System");
        Output::info("Name", &system.system);
        Output::info("System ID", &system_id_hex);
    }

    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    // Check if already deployed
    if state.systems.contains_key(&system_id_hex) {
        Output::warning(&format!("System {} already deployed", &system_id_hex[..16]));
        Output::info("System ID", &system_id_hex);
        return;
    }

    // Initialize log if needed
    if let Err(e) = BlockLog::init_if_empty() {
        Output::error(&format!("Log error: {}", e));
        return;
    }

    let config = CliConfig::load_or_default();
    let yaml = std::fs::read_to_string(yaml_path).unwrap();

    let tx = Transaction {
        sender: config.sender.clone(),
        nonce: state.get_nonce(&config.sender),
        payload: TransactionPayload::DeploySystem {
            system_id: system_id_hex.clone(),
            yaml: Some(yaml),
        },
    };

    match sequence_tx(tx, &state) {
        Ok((result,_new_state )) => {
            Output::tx_result(&result.tx_result, result.block.number, &result.block.hash, as_json);
            if result.tx_result.success {
                Output::info("System ID", &system_id_hex);
            }
        }
        Err(e) => Output::error(&format!("Sequencer error: {}", e)),
    }
}

/// Transfer assets between addresses
pub fn cmd_transfer(from: &str, to: &str, amount: u128, asset: &str, as_json: bool) {
    if !as_json {
        Output::header("Transfer");
        Output::info("From", from);
        Output::info("To", to);
        Output::info("Amount", &amount.to_string());
        Output::info("Asset", asset);
    }

    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    if let Err(e) = BlockLog::init_if_empty() {
        Output::error(&format!("Log error: {}", e));
        return;
    }

    let config = CliConfig::load_or_default();

    let asset_type = parse_asset(asset);

    let tx = Transaction {
        sender: config.sender.clone(),
        nonce: state.get_nonce(&config.sender),
        payload: TransactionPayload::Transfer {
            from: from.to_string(),
            to: to.to_string(),
            asset_type,
            amount,
        },
    };

    match sequence_tx(tx, &state) {
        Ok((result, new_state)) => {
            Output::tx_result(&result.tx_result, result.block.number, &result.block.hash, as_json);
            if result.tx_result.success && !as_json {
                let new_from = new_state.get_balance(from, &crate::types::AssetType::Eth);
                let new_to = new_state.get_balance(to, &crate::types::AssetType::Eth);
                Output::info("New From Balance", &new_from.to_string());
                Output::info("New To Balance", &new_to.to_string());
            }
        }
        Err(e) => Output::error(&format!("Sequencer error: {}", e)),
    }
}

/// Interact with a deployed system
pub fn cmd_interact(system_id: &str, mode: &str, action: Option<&str>, as_json: bool) {
    if !as_json {
        Output::header("Interact");
        Output::info("System ID", system_id);
        Output::info("Mode", mode);
    }

    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    if !state.systems.contains_key(system_id) {
        Output::error(&format!("System not found: {}", system_id));
        return;
    }

    if let Err(e) = BlockLog::init_if_empty() {
        Output::error(&format!("Log error: {}", e));
        return;
    }

    let config = CliConfig::load_or_default();

    let interact_mode = match mode {
        "evaluate" => InteractMode::EvaluateConditions,
        "trigger" => {
            match action {
                Some(a) => InteractMode::TriggerAction { action: a.to_string() },
                None => {
                    Output::error("--action required for trigger mode");
                    return;
                }
            }
        }
        "both" => {
            match action {
                Some(a) => InteractMode::Both { action: a.to_string() },
                None => {
                    Output::error("--action required for both mode");
                    return;
                }
            }
        }
        _ => {
            Output::error(&format!("Unknown mode: {}. Use: evaluate, trigger, both", mode));
            return;
        }
    };

    let tx = Transaction {
        sender: config.sender.clone(),
        nonce: state.get_nonce(&config.sender),
        payload: TransactionPayload::Interact {
            system_id: system_id.to_string(),
            mode: interact_mode,
        },
    };

    match sequence_tx(tx, &state) {
        Ok((result, _)) => {
            Output::tx_result(&result.tx_result, result.block.number, &result.block.hash, as_json);
        }
        Err(e) => Output::error(&format!("Sequencer error: {}", e)),
    }
}

/// Update an oracle value
pub fn cmd_oracle_update(system_id: &str, oracle_name: &str, value: u128, as_json: bool) {
    if !as_json {
        Output::header("Oracle Update");
        Output::info("System ID", system_id);
        Output::info("Oracle", oracle_name);
        Output::info("Value", &value.to_string());
    }

    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    if !state.systems.contains_key(system_id) {
        Output::error(&format!("System not found: {}", system_id));
        return;
    }

    if let Err(e) = BlockLog::init_if_empty() {
        Output::error(&format!("Log error: {}", e));
        return;
    }

    let config = CliConfig::load_or_default();

    let tx = Transaction {
        sender: config.sender.clone(),
        nonce: state.get_nonce(&config.sender),
        payload: TransactionPayload::OracleUpdate {
            system_id: system_id.to_string(),
            oracle_name: oracle_name.to_string(),
            value,
        },
    };

    match sequence_tx(tx, &state) {
        Ok((result, _)) => {
            Output::tx_result(&result.tx_result, result.block.number, &result.block.hash, as_json);
        }
        Err(e) => Output::error(&format!("Sequencer error: {}", e)),
    }
}

/// Print full state
pub fn cmd_state(as_json: bool) {
    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };
    Output::state(&state, as_json);
}

/// Print a specific system
pub fn cmd_state_system(system_id: &str, as_json: bool) {
    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    match state.systems.get(system_id) {
        Some(system_state) => Output::system(system_state, as_json),
        None => Output::error(&format!("System not found: {}", system_id)),
    }
}

/// Print balances for an address
pub fn cmd_state_balance(address: &str, as_json: bool) {
    let state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    let balances: Vec<(String, u128)> = state.balances
        .iter()
        .filter(|(key, _)| key.address == address)
        .map(|(key, amount)| (key.asset_id.clone(), *amount))
        .collect();

    Output::balance(address, balances, as_json);
}

/// Print block log
pub fn cmd_blocks(as_json: bool) {
    match BlockLog::read_all() {
        Ok(blocks) => Output::blocks(&blocks, as_json),
        Err(e) => Output::error(&format!("Log error: {}", e)),
    }
}

/// Rebuild state from log
pub fn cmd_replay(as_json: bool) {
    if !as_json {
        Output::header("Replaying State from Log");
    }

    match BlockLog::rebuild_state() {
        Ok(state) => {
            if !as_json {
                Output::success("State rebuilt successfully");
                Output::info("State Root", &state.state_root_hex());
                Output::info("Systems", &state.systems.len().to_string());
            } else {
                println!("{}", serde_json::to_string_pretty(&state).unwrap());
            }
        }
        Err(e) => Output::error(&format!("Replay error: {}", e)),
    }
}

/// Set sender address
pub fn cmd_config_set_sender(address: &str) {
    // Validate address format
    let re = regex::Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    if !re.is_match(address) {
        Output::error(&format!("Invalid Ethereum address: {}", address));
        return;
    }

    let config = CliConfig { sender: address.to_string() };
    match config.save() {
        Ok(_) => {
            Output::success(&format!("Sender set to {}", address));
        }
        Err(e) => Output::error(&format!("Config error: {}", e)),
    }
}

/// Show current config
pub fn cmd_config_show(as_json: bool) {
    let config = CliConfig::load_or_default();

    if as_json {
        println!("{}", serde_json::to_string_pretty(&config).unwrap());
        return;
    }

    Output::header("Config");
    Output::info("Sender", &config.sender);
}

/// Mint balance directly (for testing)
pub fn cmd_mint(address: &str, amount: u128, asset: &str, as_json: bool) {
    if !as_json {
        Output::header("Minting Balance");
        Output::info("Address", address);
        Output::info("Amount", &amount.to_string());
        Output::info("Asset", asset);
    }

    let mut state = match Store::load() {
        Ok(s) => s,
        Err(e) => { Output::error(&format!("State error: {}", e)); return; }
    };

    let asset_type = parse_asset_to_type(asset);
    state.set_balance(address, &asset_type, amount);

    match Store::save(&state) {
        Ok(_) => {
            if as_json {
                println!("{}", serde_json::json!({
                    "success": true,
                    "address": address,
                    "amount": amount.to_string(),
                    "asset": asset
                }));
            } else {
                Output::success(&format!("Minted {} {} to {}", amount, asset, address));
            }
        }
        Err(e) => Output::error(&format!("Store error: {}", e)),
    }
}

/// Parse asset string to AssetType (for state operations)
fn parse_asset_to_type(asset: &str) -> crate::types::AssetType {
    if asset.to_uppercase() == "ETH" {
        return crate::types::AssetType::Eth;
    }

    if asset.to_uppercase().starts_with("ERC20:") {
        let address = asset[6..].to_string();
        return crate::types::AssetType::Erc20 { address };
    }

    if asset.to_uppercase().starts_with("ERC721:") {
        let address = asset[7..].to_string();
        return crate::types::AssetType::Erc721 { address };
    }

    crate::types::AssetType::Eth
}

/// Parse asset string to TransactionAsset
pub fn parse_asset(asset: &str) -> TransactionAsset {
    if asset.to_uppercase() == "ETH" {
        return TransactionAsset::Eth;
    }

    if asset.to_uppercase().starts_with("ERC20:") {
        let address = asset[6..].to_string();
        return TransactionAsset::Erc20 { address };
    }

    if asset.to_uppercase().starts_with("ERC721:") {
        let address = asset[7..].to_string();
        return TransactionAsset::Erc721 { address };
    }

    // Default to ETH
    TransactionAsset::Eth
}
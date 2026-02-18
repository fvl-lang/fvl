use crate::state::{State, StateError};
use crate::system_id_from_hex;
use crate::transaction::{Transaction, TransactionPayload, InteractMode, TransactionAsset};
use crate::types::{AssetType, Expression, Action};
use crate::parser::Parser;
//use crate::hash::compute_system_id;

#[derive(Debug)]
pub struct TxResult {
    pub success: bool,
    pub error: Option<TxError>,
    pub state_root: String,
}

#[derive(Debug, thiserror::Error)]
pub enum TxError {
    #[error("Invalid nonce: {0}")]
    InvalidNonce(String),

    #[error("System not found: {0}")]
    SystemNotFound(String),

    #[error("System already deployed: {0}")]
    SystemAlreadyDeployed(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Oracle not found: {0}")]
    OracleNotFound(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
}

impl From<StateError> for TxError {
    fn from(e: StateError) -> Self {
        match e {
            StateError::SystemAlreadyDeployed(s) => TxError::SystemAlreadyDeployed(s),
            StateError::SystemNotFound(s) => TxError::SystemNotFound(s),
            StateError::OracleNotFound(s) => TxError::OracleNotFound(s),
            StateError::InsufficientBalance { address, required, available } => {
                TxError::InsufficientBalance(
                    format!("{}: required {}, has {}", address, required, available)
                )
            }
            StateError::InvalidNonce { address, expected, got } => {
                TxError::InvalidNonce(
                    format!("{}: expected {}, got {}", address, expected, got)
                )
            }
            StateError::Unauthorized(s) => TxError::Unauthorized(s),
        }
    }
}

pub struct Runtime;

impl Runtime {
fn execute(state: &mut State, tx: &Transaction) -> Result<(), TxError> {
    state.consume_nonce(&tx.sender, tx.nonce)
        .map_err(TxError::from)?;

    match &tx.payload {
        TransactionPayload::DeploySystem { system_id, yaml } => {
            let yaml_str = yaml.as_deref().unwrap_or("");
            Self::execute_deploy(state, system_id, yaml_str, &tx.sender, tx.nonce)
        }
        TransactionPayload::Interact { system_id, mode } => {
            Self::execute_interact(state, system_id, mode, &tx.sender)
        }
        TransactionPayload::OracleUpdate { system_id, oracle_name, value } => {
            Self::execute_oracle_update(state, system_id, oracle_name, *value, &tx.sender)
        }
        TransactionPayload::Transfer { from, to, asset_type, amount } => {
            Self::execute_transfer(state, from, to, asset_type, *amount, &tx.sender)
        }
    }
}

fn execute_deploy(
    state: &mut State,
    system_id: &str,
    yaml: &str,
    sender: &str,
    nonce: u64,
) -> Result<(), TxError> {
    if yaml.is_empty() {
        return Err(TxError::ParseError("Empty YAML in deploy transaction".to_string()));
    }

    let system = Parser::parse_yaml(yaml)
        .map_err(|e| TxError::ParseError(e.to_string()))?;
    let system_id_bytes = system_id_from_hex(system_id)
        .map_err(|e| TxError::ParseError(format!("Invalid system_id: {}", e)))?;

    state.deploy_system(system, system_id_bytes, sender.to_string(), nonce)
        .map_err(TxError::from)?;

    Ok(())
}

pub fn apply_tx(state: &State, tx: Transaction) -> (State, TxResult) {
        let mut new_state = state.clone();
        let result = Self::execute(&mut new_state, &tx);

    match result {
        Ok(()) => {
            let state_root = new_state.state_root_hex();
            (new_state, TxResult { success: true, error: None, state_root })
        }
        Err(e) => {
            let state_root = state.state_root_hex();
            (state.clone(), TxResult { success: false, error: Some(e), state_root })
        }
    }
}

    fn execute_interact(
        state: &mut State,
        system_id: &str,
        mode: &InteractMode,
        sender: &str,
    ) -> Result<(), TxError> {
        let system_state = state.systems.get(system_id)
            .ok_or_else(|| TxError::SystemNotFound(system_id.to_string()))?
            .clone();

        Self::check_rights(&system_state.system.rights, sender)?;

        match mode {
            InteractMode::TriggerAction { action } => {
                Self::trigger_action(state, system_id, action, sender)
            }

            InteractMode::EvaluateConditions => {
                Self::evaluate_conditions(state, system_id, sender)
            }

            InteractMode::Both { action } => {
                Self::evaluate_conditions(state, system_id, sender)?;
                Self::trigger_action(state, system_id, action, sender)
            }
        }
    }

    fn trigger_action(
        state: &mut State,
        system_id: &str,
        action_name: &str,
        sender: &str,
    ) -> Result<(), TxError> {
        let system_state = state.systems.get(system_id)
            .ok_or_else(|| TxError::SystemNotFound(system_id.to_string()))?
            .clone();

        for condition in &system_state.system.rules.conditions {
            let action_str = format!("{:?}", condition.then).to_lowercase();
            if action_str.contains(&action_name.to_lowercase()) {
                Self::apply_action(state, system_id, &condition.then.clone(), sender)?;
                return Ok(());
            }
        }

        Err(TxError::InvalidAction(action_name.to_string()))
    }

    fn evaluate_conditions(
        state: &mut State,
        system_id: &str,
        sender: &str,
    ) -> Result<(), TxError> {
        let system_state = state.systems.get(system_id)
            .ok_or_else(|| TxError::SystemNotFound(system_id.to_string()))?
            .clone();

        for condition in &system_state.system.rules.conditions {
            if Self::evaluate_expression(state, system_id, &condition.if_expr) {
                Self::apply_action(state, system_id, &condition.then.clone(), sender)?;
            }
        }

        Ok(())
    }

    fn evaluate_expression(
        state: &State,
        system_id: &str,
        expr: &Expression,
    ) -> bool {
        match expr {
            Expression::SwapRequested => true,
            Expression::LiquidityAdded => true,

            Expression::BalanceGt { value } => {
                true // MVP: simplified, expand later
            }

            Expression::CollateralRatioLt { ratio } => {
                if let Some(current) = state.get_oracle(system_id, "collateral_ratio") {
                    current < *ratio
                } else {
                    false
                }
            }

            Expression::UtilizationGt { ratio } => {
                if let Some(current) = state.get_oracle(system_id, "utilization") {
                    current > *ratio
                } else {
                    false
                }
            }

            Expression::PriceGt { oracle, value } => {
                if let Some(current) = state.get_oracle(system_id, oracle) {
                    current > *value
                } else {
                    false
                }
            }

            Expression::PriceLt { oracle, value } => {
                if let Some(current) = state.get_oracle(system_id, oracle) {
                    current < *value
                } else {
                    false
                }
            }

            Expression::PriceEq { oracle, value } => {
                if let Some(current) = state.get_oracle(system_id, oracle) {
                    current == *value
                } else {
                    false
                }
            }

            Expression::TimeGt { timestamp } => {
                true
            }

            Expression::HoldersCountGte { count } => {
                true
            }

            _ => false,
        }
    }

    /// Apply an action to state
    fn apply_action(
        state: &mut State,
        _system_id: &str,
        action: &Action,
        _sender: &str,
    ) -> Result<(), TxError> {
        match action {
            Action::Pause => {
                Ok(())
            }

            Action::Unpause => {
                Ok(())
            }

            Action::Execute { function } => {
                Ok(())
            }

            Action::ExecuteSwap => {
                Ok(())
            }

            Action::IncreaseInterestRate => {
                Ok(())
            }

            Action::Transfer { amount, from, to } => {
                let asset = AssetType::Eth; // MVP: default to ETH
                let from_balance = state.get_balance(from, &asset);

                if from_balance < *amount {
                    return Err(TxError::InsufficientBalance(
                        format!("{}: required {}, has {}", from, amount, from_balance)
                    ));
                }

                state.set_balance(from, &asset, from_balance - amount);
                let to_balance = state.get_balance(to, &asset);
                state.set_balance(to, &asset, to_balance + amount);
                Ok(())
            }

            Action::Mint { amount, to } => {
                let asset = AssetType::Eth;
                let balance = state.get_balance(to, &asset);
                state.set_balance(to, &asset, balance + amount);
                Ok(())
            }

            Action::Burn { amount, from } => {
                let asset = AssetType::Eth;
                let balance = state.get_balance(from, &asset);

                if balance < *amount {
                    return Err(TxError::InsufficientBalance(
                        format!("{}: required {}, has {}", from, amount, balance)
                    ));
                }

                state.set_balance(from, &asset, balance - amount);
                Ok(())
            }

            Action::Liquidate { target } => {
                // MVP: zero out target balance
                state.set_balance(target, &AssetType::Eth, 0);
                Ok(())
            }

            Action::Enable { permission } => Ok(()),
            Action::Disable { permission } => Ok(()),
        }
    }

    /// Execute an oracle update
    fn execute_oracle_update(
        state: &mut State,
        system_id: &str,
        oracle_name: &str,
        value: u128,
        sender: &str,
    ) -> Result<(), TxError> {
        let system_state = state.systems.get(system_id)
            .ok_or_else(|| TxError::SystemNotFound(system_id.to_string()))?
            .clone();

        if system_state.metadata.deployer != sender {
            return Err(TxError::Unauthorized(
                format!("Only deployer can update oracles for system {}", system_id)
            ));
        }

        state.set_oracle(system_id, oracle_name, value)
            .map_err(TxError::from)?;

        Ok(())
    }

    fn execute_transfer(
        state: &mut State,
        from: &str,
        to: &str,
        asset_type: &TransactionAsset,
        amount: u128,
        sender: &str,
    ) -> Result<(), TxError> {
        if sender != from {
            return Err(TxError::Unauthorized(
                format!("Sender {} cannot transfer from {}", sender, from)
            ));
        }

        let asset = transaction_asset_to_asset_type(asset_type);
        let from_balance = state.get_balance(from, &asset);

        if from_balance < amount {
            return Err(TxError::InsufficientBalance(
                format!("{}: required {}, has {}", from, amount, from_balance)
            ));
        }

        state.set_balance(from, &asset, from_balance - amount);
        let to_balance = state.get_balance(to, &asset);
        state.set_balance(to, &asset, to_balance + amount);

        Ok(())
    }

    fn check_rights(
        _rights: &std::collections::HashMap<String, Vec<String>>,
        _sender: &str,
    ) -> Result<(), TxError> {
        // MVP: anyone with any rights can interact
        Ok(())
    }
}

fn transaction_asset_to_asset_type(asset: &TransactionAsset) -> AssetType {
    match asset {
        TransactionAsset::Eth => AssetType::Eth,
        TransactionAsset::Erc20 { address } => AssetType::Erc20 { address: address.clone() },
        TransactionAsset::Erc721 { address } => AssetType::Erc721 { address: address.clone() },
        TransactionAsset::Erc1155 { address, id } => AssetType::Erc1155 {
            address: address.clone(),
            id: *id,
        },
    }
}


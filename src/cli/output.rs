use colored::*;
use prettytable::{Table, row};
use serde_json::{json, Value};
use crate::state::{State, SystemState};
use crate::block::Block;
use crate::runtime::TxResult;

pub struct Output;

impl Output {
    pub fn success(msg: &str) {
        println!("{} {}", "[OK]".green(), msg.green());
    }

    pub fn error(msg: &str) {
        eprintln!("{} {}", "[ERROR]".red(), msg.red());
    }

    pub fn warning(msg: &str) {
        println!("{} {}", "[WARN]".yellow(), msg.yellow());
    }

    pub fn info(label: &str, value: &str) {
        println!("  {}: {}", label.cyan().bold(), value.white());
    }

    pub fn header(title: &str) {
        println!("\n{}", format!("=== {} ===", title).bold().blue());
    }

    pub fn tx_result(result: &TxResult, block_number: u64, block_hash: &str, as_json: bool) {
        if as_json {
            let json = json!({
                "success": result.success,
                "block_number": block_number,
                "block_hash": block_hash,
                "state_root": result.state_root,
                "error": result.error.as_ref().map(|e| e.to_string())
            });
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
            return;
        }

        if result.success {
            Output::success("Transaction confirmed");
        } else {
            Output::error(&format!(
                "Transaction failed: {}",
                result.error.as_ref().map(|e| e.to_string()).unwrap_or_default()
            ));
        }

        Output::info("Block", &block_number.to_string());
        Output::info("Block Hash", block_hash);
        Output::info("State Root", &result.state_root);
    }

    pub fn state(state: &State, as_json: bool) {
        if as_json {
            println!("{}", serde_json::to_string_pretty(state).unwrap());
            return;
        }

        Output::header("State");

        if state.systems.is_empty() {
            println!("  {} No systems deployed", "o".dimmed());
        } else {
            println!("\n  {}", "Deployed Systems".bold().cyan());
            let mut table = Table::new();
            table.add_row(row![
                "System ID".bold(),
                "Name".bold(),
                "Deployer".bold(),
                "Deployed At".bold()
            ]);

            for (id, system_state) in &state.systems {
                table.add_row(row![
                    id[..20].to_string() + "...",
                    system_state.system.system,
                    system_state.metadata.deployer[..10].to_string() + "...",
                    system_state.metadata.deployed_at.to_string()
                ]);
            }
            table.printstd();
        }

        if state.balances.is_empty() {
            println!("\n  {} No balances", "o".dimmed());
        } else {
            println!("\n  {}", "Balances".bold().cyan());
            let mut table = Table::new();
            table.add_row(row![
                "Address".bold(),
                "Asset".bold(),
                "Balance".bold()
            ]);

            let mut balance_entries: Vec<_> = state.balances.iter().collect();
            balance_entries.sort_by(|(a, _), (b, _)| {
                a.address.cmp(&b.address).then(a.asset_id.cmp(&b.asset_id))
            });

            for (key, amount) in balance_entries {
                table.add_row(row![
                    key.address[..10].to_string() + "...",
                    key.asset_id,
                    amount.to_string()
                ]);
            }
            table.printstd();
        }

        if !state.oracles.is_empty() {
            println!("\n  {}", "Oracle Values".bold().cyan());
            let mut table = Table::new();
            table.add_row(row![
                "System ID".bold(),
                "Oracle".bold(),
                "Value".bold()
            ]);

            for (key, value) in &state.oracles {
                table.add_row(row![
                    key.system_id[..10].to_string() + "...",
                    key.oracle_name,
                    value.to_string()
                ]);
            }
            table.printstd();
        }

        println!("\n  {}: {}", "State Root".cyan().bold(), state.state_root_hex().white());
    }

    pub fn system(system_state: &SystemState, as_json: bool) {
        if as_json {
            println!("{}", serde_json::to_string_pretty(system_state).unwrap());
            return;
        }

        Output::header("System");
        Output::info("System ID", &system_state.system_id);
        Output::info("Name", &system_state.system.system);
        Output::info("Deployer", &system_state.metadata.deployer);
        Output::info("Deployed At", &system_state.metadata.deployed_at.to_string());

        println!("\n  {}", "Pool".bold().cyan());
        let pool_json = serde_json::to_string_pretty(&system_state.system)
            .unwrap_or_default();
        for line in pool_json.lines() {
            println!("    {}", line.dimmed());
        }
    }

    pub fn balance(address: &str, balances: Vec<(String, u128)>, as_json: bool) {
        if as_json {
            let entries: Vec<Value> = balances.iter().map(|(asset, amount)| {
                json!({ "asset": asset, "balance": amount.to_string() })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&json!({
                "address": address,
                "balances": entries
            })).unwrap());
            return;
        }

        Output::header(&format!("Balances for {}", &address[..10]));

        if balances.is_empty() {
            println!("  {} No balances found", "o".dimmed());
            return;
        }

        let mut table = Table::new();
        table.add_row(row!["Asset".bold(), "Balance".bold()]);
        for (asset, amount) in &balances {
            table.add_row(row![asset, amount.to_string()]);
        }
        table.printstd();
    }

    pub fn blocks(blocks: &[Block], as_json: bool) {
        if as_json {
            println!("{}", serde_json::to_string_pretty(&blocks).unwrap());
            return;
        }

        Output::header("Block Log");

        let mut table = Table::new();
        table.add_row(row![
            "#".bold(),
            "Hash".bold(),
            "Txs".bold(),
            "State Root".bold(),
            "Timestamp".bold()
        ]);

        for block in blocks {
            table.add_row(row![
                block.number.to_string(),
                block.hash[..16].to_string() + "...",
                block.txs.len().to_string(),
                block.state_root[..16].to_string() + "...",
                block.timestamp.to_string()
            ]);
        }
        table.printstd();
    }
}
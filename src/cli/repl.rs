use std::io::{self, Write};
use colored::*;
use crate::cli::commands::*;
use crate::cli::config::CliConfig;
use crate::store::Store;

pub struct Repl {
    history: Vec<String>,
    history_index: usize,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            history: vec![],
            history_index: 0,
        }
    }

    pub fn run(&mut self) {
        self.print_banner();

        loop {
            let prompt = self.build_prompt();
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    break;
                }
            }

            let input = input.trim().to_string();

            if input.is_empty() {
                continue;
            }

            // Add to history
            if self.history.last().map(|s| s.as_str()) != Some(&input) {
                self.history.push(input.clone());
            }
            self.history_index = self.history.len();

            // Handle exit
            if input == "exit" || input == "quit" || input == "q" {
                println!("{}", "Goodbye.".dimmed());
                break;
            }

            self.dispatch(&input);
        }
    }

   fn build_prompt(&self) -> String {
    let config = CliConfig::load_or_default();
    let sender_short = &config.sender[..8];

    let state_summary = match Store::load() {
        Ok(state) => format!(
            "{} systems | block {}",
            state.systems.len(),
            crate::log::BlockLog::latest()
                .ok()
                .flatten()
                .map(|b| b.number)
                .unwrap_or(0)
        ),
        Err(_) => "no state".to_string(),
    };

    format!(
        "\n{} {} {}\n{} ",
        format!("[{}...]", sender_short).cyan(),
        format!("({})", state_summary).dimmed(),
        "fvl".bold().blue(),
        ">".blue().bold()
    )
}

    fn print_banner(&self) {
        println!("{}", r#"
███████╗██╗   ██╗██╗
██╔════╝██║   ██║██║
█████╗  ██║   ██║██║
██╔══╝  ╚██╗ ██╔╝██║
██║      ╚████╔╝ ███████╗
╚═╝       ╚═══╝  ╚══════╝
"#.bold().blue());
        println!("{}", "Financial Value Language — Interactive Console".dimmed());
        println!("{}", "Type 'help' for commands, 'exit' to quit\n".dimmed());
    }

    fn dispatch(&self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let as_json = parts.contains(&"--json");
        let parts: Vec<&str> = parts.into_iter().filter(|p| *p != "--json").collect();

        match parts.as_slice() {
            ["help"] | ["h"] => self.print_help(),

            ["deploy", path] => cmd_deploy(path, as_json),

            ["transfer", from, to, amount, asset] => {
                match amount.parse::<u128>() {
                    Ok(amt) => cmd_transfer(from, to, amt, asset, as_json),
                    Err(_) => println!("{}", "Invalid amount".red()),
                }
            }

            ["interact", system_id, "evaluate"] => {
                cmd_interact(system_id, "evaluate", None, as_json)
            }

            ["interact", system_id, "trigger", action] => {
                cmd_interact(system_id, "trigger", Some(action), as_json)
            }

            ["interact", system_id, "both", action] => {
                cmd_interact(system_id, "both", Some(action), as_json)
            }

            ["oracle-update", system_id, oracle, value] => {
                match value.parse::<u128>() {
                    Ok(v) => cmd_oracle_update(system_id, oracle, v, as_json),
                    Err(_) => println!("{}", "Invalid value".red()),
                }
            }

            ["state"] => cmd_state(as_json),

            ["state", "system", system_id] => cmd_state_system(system_id, as_json),

            ["state", "balance", address] => cmd_state_balance(address, as_json),

            ["blocks"] => cmd_blocks(as_json),

            ["replay"] => cmd_replay(as_json),

            ["mint", address, amount, asset] => {
                match amount.parse::<u128>() {
                    Ok(amt) => cmd_mint(address, amt, asset, as_json),
                    Err(_) => println!("{}", "Invalid amount".red()),
                }
            }

            ["config", "set-sender", address] => cmd_config_set_sender(address),

            ["config", "show"] => cmd_config_show(as_json),

            ["history"] => {
                for (i, cmd) in self.history.iter().enumerate() {
                    println!("  {} {}", format!("{:3}.", i + 1).dimmed(), cmd);
                }
            }

                _ => {
            println!(
                "{} Unknown command: '{}'. Type {} for help.",
                "?".yellow(),
                input.yellow(),
                "help".cyan()
            );
    }
        }
    }

    fn print_help(&self) {
        println!("\n{}", "Commands".bold().cyan());

        let commands = vec![
            ("deploy <yaml-file>",                          "Deploy a new FVL system"),
            ("transfer <from> <to> <amount> <asset>",       "Transfer assets between addresses"),
            ("interact <system-id> evaluate",               "Evaluate all system conditions"),
            ("interact <system-id> trigger <action>",       "Trigger a specific action"),
            ("interact <system-id> both <action>",          "Evaluate conditions then trigger action"),
            ("oracle-update <system-id> <oracle> <value>",  "Update an oracle value"),
            ("mint <address> <amount> <asset>",             "Mint balance to address (testing)"),
            ("state",                                       "Show full state"),
            ("state system <system-id>",                    "Show a specific system"),
            ("state balance <address>",                     "Show balances for address"),
            ("blocks",                                      "Show block log"),
            ("replay",                                      "Rebuild state from block log"),
            ("config set-sender <address>",                 "Set sender address"),
            ("config show",                                 "Show current config"),
            ("history",                                     "Show command history"),
            ("help",                                        "Show this help"),
            ("exit",                                        "Exit the REPL"),
        ];

        for (cmd, desc) in commands {
            println!("  {:<50} {}", cmd.cyan(), desc.dimmed());
        }

        println!("\n{}", "Flags".bold().cyan());
        println!("  {:<50} {}", "--json".cyan(), "Output as JSON".dimmed());

        println!("\n{}", "Assets".bold().cyan());
        println!("  {:<50} {}", "ETH".cyan(), "Native ETH".dimmed());
        println!("  {:<50} {}", "ERC20:0x...".cyan(), "ERC20 token by address".dimmed());
        println!("  {:<50} {}", "ERC721:0x...".cyan(), "ERC721 NFT by address".dimmed());
    }
}
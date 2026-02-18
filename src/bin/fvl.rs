use clap::{Parser, Subcommand};
use fvl_parser::cli::repl::Repl;
use fvl_parser::cli::commands::*;

#[derive(Parser)]
#[command(
    name = "fvl",
    about = "Financial Value Language — CLI",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy a new FVL system from a YAML file
    Deploy {
        /// Path to YAML file
        yaml: String,
    },

    /// Transfer assets between addresses
    Transfer {
        from: String,
        to: String,
        amount: u128,
        /// Asset type: ETH, ERC20:0x..., ERC721:0x...
        #[arg(default_value = "ETH")]
        asset: String,
    },

    /// Interact with a deployed system
    Interact {
        /// System ID (0x...)
        system_id: String,
        /// Mode: evaluate, trigger, both
        mode: String,
        /// Action name (required for trigger and both modes)
        #[arg(long)]
        action: Option<String>,
    },

    /// Update an oracle value
    OracleUpdate {
        system_id: String,
        oracle_name: String,
        value: u128,
    },

    /// Mint balance to an address (for testing)
    Mint {
        address: String,
        amount: u128,
        #[arg(default_value = "ETH")]
        asset: String,
    },

    /// Query state
    State {
        #[command(subcommand)]
        subcommand: Option<StateCommands>,
    },

    /// Show block log
    Blocks,

    /// Rebuild state from block log
    Replay,

    /// Configuration
    Config {
        #[command(subcommand)]
        subcommand: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum StateCommands {
    /// Show a specific system
    System { system_id: String },
    /// Show balances for an address
    Balance { address: String },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set the sender address
    SetSender { address: String },
    /// Show current config
    Show,
}

fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    match cli.command {
        // No subcommand → launch REPL
        None => {
            let mut repl = Repl::new();
            repl.run();
        }

        Some(Commands::Deploy { yaml }) => cmd_deploy(&yaml, json),

        Some(Commands::Transfer { from, to, amount, asset }) => {
            cmd_transfer(&from, &to, amount, &asset, json);
        }

        Some(Commands::Interact { system_id, mode, action }) => {
            cmd_interact(&system_id, &mode, action.as_deref(), json);
        }

        Some(Commands::OracleUpdate { system_id, oracle_name, value }) => {
            cmd_oracle_update(&system_id, &oracle_name, value, json);
        }

        Some(Commands::Mint { address, amount, asset }) => {
            cmd_mint(&address, amount, &asset, json);
        }

        Some(Commands::State { subcommand }) => match subcommand {
            None => cmd_state(json),
            Some(StateCommands::System { system_id }) => cmd_state_system(&system_id, json),
            Some(StateCommands::Balance { address }) => cmd_state_balance(&address, json),
        },

        Some(Commands::Blocks) => cmd_blocks(json),

        Some(Commands::Replay) => cmd_replay(json),

        Some(Commands::Config { subcommand }) => match subcommand {
            ConfigCommands::SetSender { address } => cmd_config_set_sender(&address),
            ConfigCommands::Show => cmd_config_show(json),
        },
    }
}
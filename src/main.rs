use fvl_parser::{
    Parser, Store, BlockLog,
    Transaction, TransactionPayload, TransactionAsset, InteractMode,
    sequence_tx, compute_system_id, system_id_to_hex,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <yaml-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    println!("\n=== Parsing FVL System ===");
    let system = match Parser::parse_file(file_path) {
        Ok(s) => {
            println!("Parsed: {}", s.system);
            s
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let system_id_hex = system_id_to_hex(&compute_system_id(&system));
    println!("System ID: {}", system_id_hex);

    println!("\n=== Loading State ===");
    let mut state = match Store::load() {
        Ok(s) => {
            println!("State loaded");
            s
        }
        Err(e) => {
            eprintln!("State load error: {}", e);
            std::process::exit(1);
        }
    };

    println!("\n=== Initializing Block Log ===");
    match BlockLog::init_if_empty() {
        Ok(genesis) => println!("Genesis block: {}", genesis.hash),
        Err(e) => {
            eprintln!("Log init error: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n=== Deploying System ===");
    let yaml = std::fs::read_to_string(file_path).unwrap();
    let deployer = "0x1234567890123456789012345678901234567890".to_string();

    let deploy_tx = Transaction {
        sender: deployer.clone(),
        nonce: state.get_nonce(&deployer),
        payload: TransactionPayload::DeploySystem {
            system_id: system_id_hex.clone(),
            yaml: Some(yaml),
        },
    };

    match sequence_tx(deploy_tx, &state) {
        Ok((result, new_state)) => {
            state = new_state;
            if result.tx_result.success {
                println!("System deployed in block #{}", result.block.number);
                println!("Block hash:  {}", result.block.hash);
                println!("State root:  {}", result.block.state_root);
            } else {
                println!("Deploy tx failed: {:?}", result.tx_result.error);
            }
        }
        Err(e) => {
            eprintln!("Sequencer error: {}", e);
            std::process::exit(1);
        }
    }

    let deployed_system_id = match state.systems.keys().next() {
        Some(id) => {
            println!("Deployed System ID: {}", id);
            id.clone()
        }
        None => {
            eprintln!("System not found in state after deploy");
            std::process::exit(1);
        }
    };

    println!("\n=== Sending Transfer ===");
    let sender = "0x1234567890123456789012345678901234567890";
    let receiver = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd";

    state.set_balance(sender, &fvl_parser::AssetType::Eth, 10000);
    Store::save(&state).unwrap();

    let transfer_tx = Transaction {
        sender: sender.to_string(),
        nonce: state.get_nonce(sender),
        payload: TransactionPayload::Transfer {
            from: sender.to_string(),
            to: receiver.to_string(),
            asset_type: TransactionAsset::Eth,
            amount: 500,
        },
    };

    match sequence_tx(transfer_tx, &state) {
        Ok((result, new_state)) => {
            state = new_state;
            if result.tx_result.success {
                println!("Transfer in block #{}", result.block.number);
                println!("Block hash:  {}", result.block.hash);
                println!("State root:  {}", result.block.state_root);
                println!(
                    "Sender balance:   {}",
                    state.get_balance(sender, &fvl_parser::AssetType::Eth)
                );
                println!(
                    "Receiver balance: {}",
                    state.get_balance(receiver, &fvl_parser::AssetType::Eth)
                );
            } else {
                println!("Transfer failed: {:?}", result.tx_result.error);
            }
        }
        Err(e) => {
            eprintln!("Sequencer error: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n=== Interacting with System ===");

    let interact_tx = Transaction {
        sender: sender.to_string(),
        nonce: state.get_nonce(sender),
        payload: TransactionPayload::Interact {
            system_id: deployed_system_id.clone(),
            mode: InteractMode::EvaluateConditions,
        },
    };

    match sequence_tx(interact_tx, &state) {
        Ok((result, new_state)) => {
            state = new_state;
            if result.tx_result.success {
                println!("Interaction in block #{}", result.block.number);
                println!("Block hash:  {}", result.block.hash);
                println!("State root:  {}", result.block.state_root);
                println!("System ID:   {}", deployed_system_id);
            } else {
                println!("Interaction failed: {:?}", result.tx_result.error);
            }
        }
        Err(e) => {
            eprintln!("Sequencer error: {}", e);
            std::process::exit(1);
        }
    }

    println!("\n=== Final State ===");
    println!("State root: {}", state.state_root_hex());
    println!("\nFiles written:");
    println!("   data/state.json");
    println!("   data/blocks.log");
}

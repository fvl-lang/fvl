# FVL REPL User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Commands](#basic-commands)
4. [Advanced Usage](#advanced-usage)
5. [Complete Workflow Examples](#complete-workflow-examples)
6. [Command Reference](#command-reference)
7. [Tips and Best Practices](#tips-and-best-practices)

---

## Introduction

The FVL (Financial Value Language) REPL is an interactive command-line interface for deploying and interacting with financial systems on the FVL rollup. Think of it as a financial systems playground where you can:

- Deploy financial systems from YAML templates
- Transfer assets between addresses
- Interact with deployed systems
- Query state and balances
- Manage oracle data feeds

---

## Getting Started

### Prerequisites

Before using the REPL, you need to set up the local infrastructure:

**Terminal 1 - Start Anvil (Local Ethereum) need Foundry installed**
```bash
anvil
```

**Terminal 2 - Deploy Settlement Contract**
```bash
bash contracts/deploy.sh
```

This deploys the FvlRollup contract to Anvil and writes the address to `data/contract.json`.

**Terminal 3 - Start State Root Submitter (Optional)**
```bash
# Submit state roots every 1 blocks, poll every 10 seconds - Tune to liking
FVL_SUBMIT_INTERVAL=1 FVL_POLL_INTERVAL=10 cargo run --bin submitter
```

The submitter watches for new blocks and anchors state roots to Ethereum.

### Launching the REPL

**Terminal 4 - Start REPL**
```bash
cargo run --bin fvl
```

You'll see the FVL banner and an interactive prompt:

```
[0xf39Fd6...] (0 systems | block 0) fvl
> 
```

The prompt shows:
- Your current sender address (first 8 chars)
- Number of deployed systems
- Current block height

### First Steps

1. Check available commands:
```bash
> help
```

2. View current configuration:
```bash
> config show
```

3. Set your sender address (if needed):
```bash
> config set-sender 0xYourAddressHere
```

---

## Basic Commands

### 1. Deploying a System

Deploy a financial system from a YAML file:

```bash
> deploy tests/fixtures/simple_swap.yaml
```

Output:
```
=== Deploying System ===
  Name: SimpleSwap
  System ID: 0xaf925603c9dbb726822239bbc08ec16f175d3f49581c3922c3b9b9b39ef77a63

[OK] Transaction confirmed
  Block: 1
  Block Hash: 0x3f43b83b9b99afa3893c9473b938ca5ef813ade7b13a0d25279d003820c4d649
  State Root: 0x1568fe03f5721dfe0015ad393d97f60f609c33eb441a73df2d7860514eafe5dd
```

Take note of the System ID - you'll need it for interactions.

### 2. Minting Balance (Testing)

For testing, you can mint balances directly:

```bash
> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 10000 ETH
```

Asset types:
- `ETH` - Native ETH
- `ERC20:0xTokenAddress` - ERC20 tokens
- `ERC721:0xNFTAddress` - ERC721 NFTs

### 3. Transferring Assets

Transfer between addresses:

```bash
> transfer 0xFromAddress 0xToAddress 500 ETH
```

Example with ERC20:
```bash
> transfer 0xFrom 0xTo 1000 ERC20:0xUSDCAddress
```

### 4. Checking State

View all state:
```bash
> state
```

View balances for a specific address:
```bash
> state balance 0xYourAddress
```

View a specific system:
```bash
> state system 0xSystemID
```

### 5. Viewing Blocks

See the blockchain history:
```bash
> blocks
```

Output shows:
- Block number
- Block hash
- Transaction count
- State root
- Timestamp

---

## Advanced Usage

### Interacting with Systems

There are three interaction modes:

#### Mode 1: Evaluate Conditions
Runs all conditions in the system and executes matching actions:

```bash
> interact 0xSystemID evaluate
```

#### Mode 2: Trigger Specific Action
Triggers a named action directly:

```bash
> interact 0xSystemID trigger swap
```

#### Mode 3: Both
Evaluates all conditions first, then triggers a specific action:

```bash
> interact 0xSystemID both add_liquidity
```

### Oracle Updates

Update data feeds for systems that use oracles:

```bash
> oracle-update 0xSystemID eth_price 3000
```

This updates the `eth_price` oracle to 3000 for the specified system.

### JSON Output

Any command can output JSON instead of formatted text:

```bash
> state --json
```

```bash
> blocks --json
```

This is useful for:
- Scripting and automation
- Parsing output programmatically
- Debugging

### Replaying State

Rebuild state from the block log (useful after corruption or for verification):

```bash
> replay
```

This reads all blocks and recomputes the state from scratch.

### Command History

View your command history:

```bash
> history
```

Output:
```
  1. deploy tests/fixtures/simple_swap.yaml
  2. mint 0xf39Fd6... 10000 ETH
  3. transfer 0xf39... 0xabcd... 500 ETH
  4. state
```

---

## Complete Workflow Examples

### Example 1: Simple AMM Deployment and Usage

```bash
# Step 1: Deploy the AMM
> deploy tests/fixtures/simple_swap.yaml

# Step 2: Note the System ID from output
# System ID: 0xaf925603c9dbb726822239bbc08ec16f175d3f49581c3922c3b9b9b39ef77a63

# Step 3: Mint liquidity for yourself
> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 100000 ETH

# Step 4: Check your balance
> state balance 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

# Step 5: Interact with the AMM
> interact 0xaf925603c9dbb726822239bbc08ec16f175d3f49581c3922c3b9b9b39ef77a63 evaluate

# Step 6: View updated state
> state
```

### Example 2: Lending Pool with Oracles

```bash
# Deploy lending pool
> deploy tests/fixtures/lending_pool.yaml

# System ID: 0x1a2b3c4d5e6f...

# Mint collateral
> mint 0xBorrowerAddress 50000 ETH

# Set ETH price oracle
> oracle-update 0x1a2b3c4d5e6f... eth_price 2500

# Check if liquidation conditions are met
> interact 0x1a2b3c4d5e6f... evaluate

# Update price (simulate price drop)
> oracle-update 0x1a2b3c4d5e6f... eth_price 1800

# Trigger liquidation evaluation
> interact 0x1a2b3c4d5e6f... evaluate

# View final state
> state system 0x1a2b3c4d5e6f...
```

### Example 3: Multi-User Transfer Flow

```bash
# Setup: Mint to User A
> mint 0xUserA 5000 ETH

# User A transfers to User B
> config set-sender 0xUserA
> transfer 0xUserA 0xUserB 1000 ETH

# User B transfers to User C
> config set-sender 0xUserB
> transfer 0xUserB 0xUserC 300 ETH

# Check final balances
> state balance 0xUserA
> state balance 0xUserB
> state balance 0xUserC

# View transaction history
> blocks
```

### Example 4: System Comparison

```bash
# Deploy multiple systems
> deploy system_a.yaml
> deploy system_b.yaml

# View all deployed systems
> state

# Compare specific systems
> state system 0xSystemA_ID
> state system 0xSystemB_ID

# Export for analysis
> state --json > state_snapshot.json
```

---

## Command Reference

### System Management

| Command | Syntax | Description |
|---------|--------|-------------|
| deploy | `deploy <yaml-file>` | Deploy a new FVL system |
| state system | `state system <system-id>` | View details of a specific system |

### Asset Management

| Command | Syntax | Description |
|---------|--------|-------------|
| mint | `mint <address> <amount> <asset>` | Mint balance to address (testing) |
| transfer | `transfer <from> <to> <amount> <asset>` | Transfer assets between addresses |
| state balance | `state balance <address>` | View balances for an address |

### System Interaction

| Command | Syntax | Description |
|---------|--------|-------------|
| interact | `interact <system-id> evaluate` | Evaluate all system conditions |
| interact | `interact <system-id> trigger <action>` | Trigger a specific action |
| interact | `interact <system-id> both <action>` | Evaluate then trigger |
| oracle-update | `oracle-update <system-id> <oracle> <value>` | Update oracle value |

### State Queries

| Command | Syntax | Description |
|---------|--------|-------------|
| state | `state` | Show full state |
| blocks | `blocks` | Show block log |
| replay | `replay` | Rebuild state from log |

### Configuration

| Command | Syntax | Description |
|---------|--------|-------------|
| config show | `config show` | Display current config |
| config set-sender | `config set-sender <address>` | Set sender address |

### Utility

| Command | Syntax | Description |
|---------|--------|-------------|
| help | `help` or `h` | Show all commands |
| history | `history` | View command history |
| exit | `exit`, `quit`, or `q` | Exit the REPL |

### Global Flags

| Flag | Description |
|------|-------------|
| `--json` | Output as JSON instead of formatted text |

---

## Tips and Best Practices

### 1. Save System IDs

When deploying systems, always copy the System ID. You'll need it for all interactions:

```bash
# Good practice: Keep a note
System: SimpleSwap
ID: 0xaf925603c9dbb726822239bbc08ec16f175d3f49581c3922c3b9b9b39ef77a63
```

### 2. Use Short Addresses for Testing

Anvil (local testnet) provides default addresses:

```
0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266  # Account 0
0x70997970C51812dc3A010C7d01b50e0d17dc79C8  # Account 1
0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC  # Account 2
```

### 3. Check State After Every Transaction

Get in the habit of verifying state changes:

```bash
> transfer 0xFrom 0xTo 500 ETH
> state balance 0xFrom
> state balance 0xTo
```

### 4. Use JSON for Scripting

When building scripts or tools, use JSON output:

```bash
> state --json | jq '.balances'
> blocks --json | jq 'map(select(.number > 5))'
```

### 5. Replay Before Important Operations

If you suspect state corruption, replay first:

```bash
> replay
> state
```

### 6. Test Oracle Updates

Before deploying to production, test oracle behavior:

```bash
# Set normal price
> oracle-update 0xSystem eth_price 2500
> interact 0xSystem evaluate

# Set extreme price
> oracle-update 0xSystem eth_price 100
> interact 0xSystem evaluate
```

### 7. Use History for Repetitive Tasks

Instead of retyping, use history to repeat commands:

```bash
> history
  5. oracle-update 0x1a2b3c... eth_price 2500

# Then manually retype line 5 with modifications
```

### 8. Monitor Block Production

Periodically check blocks to ensure the sequencer is producing:

```bash
> blocks
```

If block production stops, the sequencer may have crashed.

### 9. Understand Asset Identifiers

- `ETH` - Always capitalized
- `ERC20:0x...` - Full address required
- `ERC721:0x...` - Full address required

### 10. Exit Gracefully

Always use `exit` instead of Ctrl+C to ensure proper cleanup:

```bash
> exit
```

---

## Troubleshooting

### System Not Found Error

```
[ERROR] System not found: 0x...
```

**Solution:** Double-check the System ID. Use `state` to list all deployed systems.

### Invalid Nonce Error

```
[ERROR] Invalid nonce: expected 5, got 3
```

**Solution:** Your sender's nonce is out of sync. This usually means you changed senders mid-session. Check:
```bash
> config show
```

### Insufficient Balance

```
[ERROR] Insufficient balance: required 500, has 100
```

**Solution:** Mint more balance:
```bash
> mint 0xYourAddress 10000 ETH
```

### Parse Error

```
[ERROR] Parse error: missing field `pool`
```

**Solution:** Your YAML file is malformed. Validate it against the FVL schema.

### Unauthorized Oracle Update

```
[ERROR] Unauthorized: Only deployer can update oracles
```

**Solution:** Only the address that deployed the system can update its oracles. Set the correct sender:
```bash
> config set-sender 0xDeployerAddress
```

---

## Verifying Ethereum Settlement

### Checking On-Chain State Roots

After the submitter has been running, you can verify that state roots are being anchored to Ethereum.

**Using cast (in a new terminal):**
```bash
# Get the contract address
CONTRACT=$(jq -r '.address' data/contract.json)

# Check latest state root on-chain
cast call $CONTRACT "latestStateRoot()(bytes32)" --rpc-url http://localhost:8545

# Check latest block number
cast call $CONTRACT "latestBlockNumber()(uint256)" --rpc-url http://localhost:8545

# Get both at once
cast call $CONTRACT "getLatest()(uint256,bytes32)" --rpc-url http://localhost:8545
```

**Comparing with local state:**
```bash
# In REPL
> state

# Note the State Root value, then check if it matches on-chain
```

### Complete End-to-End Verification

```bash
# Terminal 1: Anvil running
# Terminal 2: deploy.sh already ran
# Terminal 3: Submitter running

# Terminal 4: In REPL
> deploy tests/fixtures/simple_swap.yaml
> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 10000 ETH
> transfer 0xf39Fd6... 0xabcd... 500 ETH
> transfer 0xf39Fd6... 0xabcd... 500 ETH
> transfer 0xf39Fd6... 0xabcd... 500 ETH
> transfer 0xf39Fd6... 0xabcd... 500 ETH
> transfer 0xf39Fd6... 0xabcd... 500 ETH

# Wait for submitter to submit (check Terminal 3 output)

# Terminal 5: Verify on Ethereum
cast call $CONTRACT "getLatest()(uint256,bytes32)" --rpc-url http://localhost:8545

# You should see the block number and state root match your REPL state
```

This demonstrates the full Optimistic Rollup flow:
1. Transactions executed off-chain (FVL runtime)
2. Blocks produced (sequencer)
3. State roots anchored on-chain (Ethereum settlement)

---

## Quick Reference Card

```
DEPLOY          deploy <file>
MINT            mint <addr> <amt> <asset>
TRANSFER        transfer <from> <to> <amt> <asset>
INTERACT        interact <sys-id> <mode> [--action <name>]
ORACLE          oracle-update <sys-id> <oracle> <value>

STATE           state
BALANCE         state balance <addr>
SYSTEM          state system <sys-id>
BLOCKS          blocks
REPLAY          replay

CONFIG          config show | set-sender <addr>
HELP            help
HISTORY         history
EXIT            exit

FLAGS           --json (for any command)
```

---

## Next Steps

Now that you're familiar with the REPL:

1. Explore the example YAML templates in `tests/fixtures/`
2. Create your own financial system templates
3. Experiment with different interaction modes
4. Build complex multi-system workflows
5. Integrate with external tools using JSON output

For more information on creating FVL systems, see the FVL Language Specification.
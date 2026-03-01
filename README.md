# FVL: Financial Value Language

A declarative execution layer for financial coordination on Ethereum.

-----

## The Problem with DeFi Security

Most DeFi vulnerabilities don’t originate in financial logic. They originate in the arbitrary computation surrounding it. When you implement a declarative pattern in a Turing-complete environment, you inherit the entire attack surface of that environment without needing any of its power.

Reentrancy exploits, integer overflows, unbounded loop attacks — these aren’t failures of financial intent. They’re failures that emerge at the gap between financial intent and general-purpose execution.

## The Boundary

FVL draws a formal line.

If a financial system’s state transitions are fully deterministic and expressible over known primitives — it belongs on FVL. If it requires arbitrary computation whose values cannot be known before execution — it belongs on Ethereum.

This is the declarative/imperative boundary applied to financial systems. FVL doesn’t try to replace Ethereum. It defines the subset of financial coordination that should never have required Turing completeness.

## What This Looks Like

```yaml
system: "CommunityStaking"

pool:
  collect:
    from:
      type: token_holders
      address: "0xYourToken"
    what:
      type: erc20
      address: "0xStakingToken"
    min:
      type: value
      amount: "100"
    cap:
      type: value
      amount: "1000000"

rules:
  conditions:
    - if:
        type: time_gt
        timestamp: "1735689600"
      then:
        type: enable
        permission: withdraw
  
  distribute:
    formula:
      type: proportional
    to:
      type: contributors
    triggers: continuous

time:
  locks:
    type: duration
    seconds: "2592000"
  vesting:
    type: linear
    duration: "7776000"

rights:
  contributors: [stake, unstake]
  admin: [pause, update_params]

oracles: []
```

This is the system — not a configuration file passed to a smart contract. FVL parses, validates, and executes it directly. No Solidity. No implementation layer. No surface area between intent and execution.

-----

## Primitives

### Access Control

- `anyone` — open to all
- `token_holders(address)` — ERC20 gated
- `nft_holders(address)` — NFT gated
- `whitelist([addresses])` — explicit allow list
- `min_balance(amount, token)` — minimum holdings

### Assets

- `eth` — native Ethereum
- `erc20(address)` — fungible tokens
- `erc721(address)` — NFTs
- `erc1155(address, id)` — semi-fungibles
- `multiple([...])` — multi-asset systems

### Conditions

- `balance > X`
- `time > timestamp`
- `price(feed) < threshold`
- `holders >= N`
- `total_value == cap`

### Actions

- `enable(permission)` / `disable(permission)`
- `liquidate(target)`
- `mint(amount, to)` / `burn(amount, from)`
- `transfer(amount, from, to)`

### Distribution

- `proportional` — by stake size
- `equal` — even split
- `weighted(metric)` — custom weighting
- `tiered(thresholds)` — bracket-based
- `quadratic` — quadratic funding

### Time

- `linear(duration)` — vesting over time
- `cliff(duration)` — unlock after period
- `graded(schedule)` — milestone-based

-----

## Architecture

```
┌─────────────────────────────────────────┐
│           User Templates (YAML)          │
└─────────────────┬───────────────────────┘
                  ▼
┌─────────────────────────────────────────┐
│         FVL Parser & Validator           │
└─────────────────┬───────────────────────┘
                  ▼
┌─────────────────────────────────────────┐
│          Runtime Engine (Rust)           │
│   (Deterministic state transitions)      │
└─────────────────┬───────────────────────┘
                  ▼
┌─────────────────────────────────────────┐
│           Block Production               │
│      (Sequencer, append-only log)        │
└─────────────────┬───────────────────────┘
                  ▼
┌─────────────────────────────────────────┐
│      Ethereum Settlement Layer           │
└─────────────────────────────────────────┘
```

**Deterministic execution** — same transactions, same order, identical state. Always.

**Constrained language** — no loops, recursion, or unbounded computation. Attack surface minimized by construction, not by auditing.

**Composable systems** — systems reference other systems. Primitives are audited once, reused without re-auditing.

**Ethereum settlement** — state roots anchored on L1. Inherits Ethereum security at the settlement layer.

-----

## Quick Start

**Prerequisites:** Rust 1.75+, Foundry

```bash
git clone https://github.com/fvl-lang/fvl.git
cd fvl
cargo build --release
cargo test
```

```bash
# Terminal 1
anvil

# Terminal 2
bash contracts/deploy.sh

# Terminal 3
cargo run --bin fvl
```

```bash
> config set-sender 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
> deploy tests/fixtures/simple_swap.yaml
> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 10000 ETH
> transfer 0xf39Fd6... 0xabcdef... 500 ETH
> state
> blocks
```

-----

## Current Status: MVP

**Working:** YAML system deployment, deterministic state computation, block production, state root generation, Ethereum settlement, interactive REPL, CLI with JSON output.

**Not yet:**

- Fraud proofs (centralized sequencer, cannot challenge invalid state)
- L1↔L2 asset bridge (internal balances only)
- Decentralized sequencer

Not production ready. MVP validates the thesis and the primitive set.

-----

## Contributing

The most useful contributions at this stage are boundary challenges — financial systems that appear declarative but cannot be expressed with current primitives. These either surface missing primitives or sharpen the boundary definition.

- **Design partners** — protocols willing to attempt real system deployment and report where primitives break down
- **Template creators** — verified templates for common patterns
- **Contributors** — primitives, tooling, documentation

Primitive additions are held to a high standard. Every addition expands the attack surface.

See <CONTRIBUTING.md> for guidelines.

-----

## Documentation

- [Language Specification](docs/FVL_SPEC.md)
- [REPL Guide](docs/FVL_REPL_Guide.md)
- [Project Overview](docs/FVL_PROJECT_OVERVIEW.md)
- [Template Examples](tests/fixtures/)

-----

## Community

**Twitter/X:** [@FVL_Finance](https://twitter.com/fvl_finance) · **Discord:** [discord.gg/uRrtJQrp](https://discord.gg/uRrtJQrp)

-----

## License

GPL-3.0. See <LICENSE>.

-----

*Built on Ethereum Foundation, Optimism rollup architecture, and Foundry tooling.*
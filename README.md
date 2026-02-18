# FVL: Financial Value Language

**Build financial systems from composable blocks. No code required.**

FVL is an Optimistic Rollup purpose-built for financial coordination. Deploy sophisticated treasury management, staking mechanisms, lending pools, and crowdfunding systems by combining simple, verified building blocks.

---

## Why FVL?

Most Layer 2s scale Ethereum. **FVL specializes it.**

Traditional DeFi requires:
- Writing Solidity smart contracts
- Months of development
- Expensive security audits
- Deep technical expertise

FVL provides:
- Declarative YAML templates
- Deploy in minutes
- Verified primitives (audit once, reuse forever)
- Accessible to non-developers

**Financial systems are patterns. FVL gives you the building blocks.**

---

## Example: Custom Staking System

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

**That's it.** Deploy this template and you have a fully functional staking system with:
- Token-gated access
- Time-locked withdrawals
- Proportional reward distribution
- Linear vesting
- Role-based permissions

No Solidity. No audits. No vulnerabilities from implementation bugs.

---

## The Building Blocks

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
- Balance checks: `balance > X`
- Time triggers: `time > timestamp`
- Oracle data: `price(feed) < threshold`
- Holder counts: `holders >= N`
- Value tracking: `total_value == cap`

### Actions
- `enable(permission)` — grant access
- `disable(permission)` — revoke access
- `liquidate(target)` — enforce penalties
- `mint(amount, to)` — create assets
- `burn(amount, from)` — destroy assets
- `transfer(amount, from, to)` — move value

### Distribution Formulas
- `proportional` — by stake size
- `equal` — even split
- `weighted(metric)` — custom weighting
- `tiered(thresholds)` — bracket-based
- `quadratic` — quadratic funding

### Time Mechanics
- `linear(duration)` — vesting over time
- `cliff(duration)` — unlock after period
- `graded(schedule)` — milestone-based
- Lock periods and release schedules

**Combine these blocks to create any financial system imaginable.**

---

## Quick Start

### Prerequisites

- Rust 1.75+
Linux/macOS
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Windows
> https://win.rustup.rs/x86_64

- Foundry (for local Ethereum node)
> curl -L https://foundry.paradigm.xyz | bash && foundryup

### Installation

```bash
# Clone the repository
git clone https://github.com/fvl-lang/fvl.git
cd fvl

# Build
cargo build --release

# Run tests
cargo test
```

### Start Local Infrastructure

```bash
# Terminal 1: Start local Ethereum node (Need Foundry installed)
anvil

# Terminal 2: Deploy settlement contract
bash contracts/deploy.sh

# Terminal 3: Start the REPL
cargo run --bin fvl
```

### Deploy Your First System

```bash
# In the FVL REPL
> config set-sender 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
> deploy tests/fixtures/simple_swap.yaml

# Mint some test balance
> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 10000 ETH

# Transfer assets
> transfer 0xf39Fd6... 0xabcdef... 500 ETH

# Check state
> state

# View blocks
> blocks
```

**You just deployed a financial system to an L2.**

---

## Use Cases

### Community Treasuries
Deploy custom treasury management with:
- Multi-condition spending rules
- Tiered approval thresholds
- Vesting for team allocations
- Transparent on-chain logic

### Custom Staking
Create unique staking mechanisms:
- Dynamic reward formulas
- Lock period requirements
- Performance-based bonuses
- Integration with price oracles

### Lending Pools
Launch lending systems with:
- Custom collateralization ratios
- Oracle-based liquidations
- Interest rate curves
- Risk-adjusted parameters

### Crowdfunding
Build fundraising campaigns:
- Min/max contribution limits
- Refund conditions
- Milestone-based releases
- Automated distributions

### Yield Aggregators
Compose systems that:
- Route deposits to highest yield
- Rebalance based on conditions
- Compound rewards automatically
- Distribute profits proportionally

**Financial systems limited only by imagination.**

---

## Architecture

```
┌─────────────────────────────────────────┐
│           User Templates (YAML)          │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│         FVL Parser & Validator           │
│  (Validates syntax, generates system ID) │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│          Runtime Engine (Rust)           │
│   (Deterministic state transitions)      │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│           Block Production               │
│      (Sequencer, append-only log)        │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│      Ethereum Settlement Layer           │
│    (State roots anchored on mainnet)     │
└─────────────────────────────────────────┘
```

### Key Properties

**Deterministic Execution**
- Given the same transactions in the same order, state is always identical
- Perfect replayability
- Independent verification

**Constrained Language**
- Only financial primitives allowed
- No loops, recursion, or unbounded computation
- Attack surface minimized by design

**Composable Systems**
- Systems can reference other systems
- Build complex coordination from simple blocks
- Unlimited depth of composition

**Ethereum Settlement**
- State roots anchored on L1
- Full transaction data on-chain
- Inherits Ethereum security guarantees

---

## Current Status: MVP

### What Works Today

- Deploy YAML financial systems
- Execute transactions (deploy, transfer, interact, oracle updates)
- Deterministic state computation
- Block production and ordering
- State root generation
- Ethereum settlement (state root submission)
- Interactive REPL for system interaction
- Command-line interface with JSON output

### Known Limitations (Roadmap Items)

**Trust Assumptions:**
- Centralized sequencer (users must trust operator)
- No fraud proofs yet (cannot challenge invalid state)
- No forced inclusion (sequencer can censor)

**Asset Limitations:**
- No L1↔L2 bridge (internal balances only)
- Cannot deposit/withdraw real assets
- Testing/demo environment only


---

## Contributing

We're actively seeking:
- **Design partners** — DAOs/protocols willing to deploy real systems and provide feedback
- **Template creators** — build and share financial system templates
- **Contributors** — improvements to primitives, tooling, documentation

### Areas of Focus

1. **Primitive Design** — which building blocks are missing?
2. **Template Library** — verified templates for common patterns
3. **Developer Tools** — better debugging, testing, visualization
4. **Documentation** — guides, tutorials, examples

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Documentation

- [FVL Language Specification](docs/FVL_SPEC.md) — complete primitive reference
- [REPL User Guide](docs/FVL_REPL_Guide.md) — interactive console tutorial
- [FVL Overview](docs/FVL_PROJECT_OVERVIEW.md) — technical deep dive
- [Template Examples](tests/fixtures/) — example systems

---

## FAQ

### Is this an L2?

FVL is an Optimistic Rollup that settles to Ethereum. Currently in MVP stage:
- Transactions are executed off-chain
- State roots are submitted to Ethereum 
- Full transaction data is published (data availability)
- Missing fraud proofs (planned for next phase)

### How is this different from other L2s?

Most L2s (Optimism, Arbitrum) scale Ethereum by providing cheaper EVM execution.

FVL **specializes** Ethereum by providing a domain-specific language for financial systems.

Not better or worse — different purpose.

### Can I use this in production?

**Not yet.** MVP is for:
- Testing concepts
- Prototyping systems
- Validating primitives
- Building template library

Production use requires:
- Fraud proof system
- Asset bridge
- Decentralized sequencer


### Why YAML instead of Solidity?

Financial systems follow predictable patterns. YAML:
- Makes patterns explicit
- Enforces safety through constraints
- Enables verification without auditing code
- Accessible to non-developers

Solidity is Turing-complete — powerful but dangerous for financial coordination.

### What about composability with existing DeFi?

Post-MVP will include:
- Asset bridge for L1↔L2 transfers
- Oracle integration (Chainlink, etc.)
- Cross-system references within FVL

Composability with external DeFi protocols requires bridge infrastructure.

### How do I propose a new primitive?

Open an issue with:
- Use case that can't be expressed with current primitives
- Proposed syntax
- Security considerations
- Why it's fundamental (not a special case)

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Primitive additions are carefully evaluated to maintain language coherence.

---

## Community

- **Twitter/X:** [@FVL_Finance](https://twitter.com/fvl_finance)
- **Discord:** [discord.gg/uRrtJQrp](https://discord.gg/uRrtJQrp) // PATCH

---

## License

Licensed under the GNU General Public License v3.0 (GPLv3).

This means:
- You can use, modify, and distribute this software
- Any derivative work must also be open source under GPLv3
- Commercial use is allowed
- You must disclose source code of any modifications

See [LICENSE](LICENSE) for full terms.

---

## Acknowledgments

Built on the shoulders of:
- Ethereum Foundation (settlement layer)
- Optimism (rollup architecture inspiration)
- Foundry (development tooling)

---

**FVL: Financial systems as composable building blocks.**

Not just another L2. A specialized platform for financial coordination.

[Get Started](#quick-start) | [View Examples](tests/fixtures/) | [Read Docs](docs/) | [Join Community](#community)
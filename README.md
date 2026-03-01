
The deeper implication of that boundary is significant. Most financial systems that have been exploited historically - the DAO hack, various DeFi exploits - were exploited precisely at the arbitrary computation boundary. The vulnerability wasn’t in the financial logic, it was in the imperative execution layer surrounding it. FVL’s thesis is that if the financial logic is fully declarative and deterministic, that entire class of exploit surface disappears by construction.

That’s a security argument, not just an accessibility argument. And it’s stronger than the accessibility framing you lead with in the README.

Which raises a presentation question - the README currently frames FVL primarily around accessibility, no Solidity required, deploy in minutes, accessible to non-developers. That’s a valid entry point but it undersells the deeper architectural argument. The sophisticated audience you’re seeking feedback from will find the determinism-as-security-boundary argument more intellectually compelling than the accessibility argument.

The accessibility is a consequence of the architectural decision. The architectural decision is the thesis.

# FVL: Financial Value Language

**A declarative execution layer for financial coordination on Ethereum.**

Most DeFi vulnerabilities don’t originate in financial logic. They originate in the arbitrary computation surrounding it. FVL eliminates that surface entirely by drawing a formal boundary: if a financial system can be fully described as deterministic state transitions over known primitives, it belongs on FVL. If it requires arbitrary computation whose values are unknown before execution, it belongs on Ethereum.

This boundary is not a limitation. It is the thesis.

-----

## The Core Argument

Ethereum is Turing-complete by design. That generality is powerful and necessary for systems that require arbitrary computation. But most financial coordination — staking, treasury management, lending pools, crowdfunding, vesting schedules — does not require arbitrary computation. These systems are patterns. Their state transitions are fully describable in advance. Their logic is declarative, not imperative.

When you implement a declarative pattern in a Turing-complete environment, you inherit the entire attack surface of that environment without needing any of its power. The DAO hack, reentrancy exploits, integer overflow vulnerabilities — these are not failures of financial logic. They are failures that occur at the boundary between financial intent and arbitrary execution.

FVL removes that boundary by building a purpose-specific execution environment where only financial primitives exist. No loops. No recursion. No unbounded computation. Given the same transactions in the same order, state is always identical. The system is fully replayable and independently verifiable by construction.

**FVL does not try to replace Ethereum. It defines the subset of financial systems that should never have required it.**

-----

## The Boundary Defined

A system belongs on FVL if:

- Its state transitions are fully deterministic and describable before execution
- Its logic can be expressed as conditions over known primitives
- Its values are known or oracle-sourced, not computed on the fly

A system belongs on Ethereum if:

- It requires arbitrary computation whose output cannot be known in advance
- It requires loops, recursion, or unbounded processing
- Its logic cannot be expressed declaratively without loss of correctness

This is a formal distinction, not a practical convenience. It maps directly onto the declarative versus imperative boundary in programming language theory.

-----

## What This Means in Practice

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

This is a fully functional staking system with token-gated access, time-locked withdrawals, proportional reward distribution, linear vesting, and role-based permissions. It is not a configuration file passed to a smart contract. It is the system. The execution environment parses, validates, and runs it directly — no Solidity, no implementation layer, no surface area between intent and execution where vulnerabilities emerge.

-----

## The Primitive Set

FVL’s primitives are deliberately constrained to what financial coordination actually requires. Primitive additions are evaluated against one criterion: is this a fundamental building block of financial coordination, or is it a special case that should be composed from existing primitives?

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

-----

## Use Cases

The following systems are fully expressible in FVL because their state transitions are deterministic and their logic is declarative.

**Community Treasuries** — multi-condition spending rules, tiered approval thresholds, vesting for team allocations, transparent on-chain logic.

**Custom Staking** — dynamic reward formulas, lock period requirements, performance-based bonuses, oracle integration.

**Lending Pools** — custom collateralization ratios, oracle-based liquidations, interest rate curves, risk-adjusted parameters.

**Crowdfunding** — min/max contribution limits, refund conditions, milestone-based releases, automated distributions.

**Yield Aggregators** — deposit routing, condition-based rebalancing, automatic compounding, proportional profit distribution.

-----

## Architecture

```
┌─────────────────────────────────────────┐
│           User Templates (YAML)          │
│     (Declarative financial intent)       │
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

**Deterministic Execution** — given the same transactions in the same order, state is always identical. Perfect replayability. Independent verification without trust assumptions about the execution environment.

**Constrained Language** — only financial primitives are expressible. No loops, recursion, or unbounded computation. The attack surface is minimized by construction, not by auditing.

**Composable Systems** — systems can reference other systems. Complex coordination emerges from simple, verified blocks. Primitives are audited once and reused without re-auditing.

**Ethereum Settlement** — state roots anchored on L1. Full transaction data on-chain. Inherits Ethereum security guarantees at the settlement layer.

-----

## Quick Start

### Prerequisites

- Rust 1.75+

Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows

```
https://win.rustup.rs/x86_64
```

- Foundry (for local Ethereum node)

```bash
curl -L https://foundry.paradigm.xyz | bash && foundryup
```

### Installation

```bash
git clone https://github.com/fvl-lang/fvl.git
cd fvl
cargo build --release
cargo test
```

### Start Local Infrastructure

```bash
# Terminal 1: Start local Ethereum node
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

> mint 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 10000 ETH
> transfer 0xf39Fd6... 0xabcdef... 500 ETH
> state
> blocks
```

-----

## Current Status: MVP

### What Works Today

- Deploy YAML financial systems
- Execute transactions (deploy, transfer, interact, oracle updates)
- Deterministic state computation
- Block production and ordering
- State root generation
- Ethereum settlement (state root submission)
- Interactive REPL
- CLI with JSON output

### Known Limitations

**Trust Assumptions**

- Centralized sequencer (users must trust operator)
- No fraud proofs yet (cannot challenge invalid state)
- No forced inclusion (sequencer can censor)

**Asset Limitations**

- No L1↔L2 bridge (internal balances only)
- Cannot deposit/withdraw real assets
- Testing and prototyping environment only

-----

## Contributing

The most valuable contributions at this stage are design challenges — financial systems that appear to fall within FVL’s boundary but cannot be expressed with current primitives. These either reveal missing primitives or sharpen the boundary definition.

We are actively seeking:

- **Design partners** — DAOs and protocols willing to attempt real system deployment and report where the primitive set breaks down
- **Template creators** — verified templates for common financial patterns
- **Contributors** — improvements to primitives, tooling, documentation

### Areas of Focus

1. **Primitive completeness** — which financial coordination patterns cannot be expressed declaratively with current primitives?
1. **Boundary stress testing** — cases that sit at the edge of the declarative/imperative boundary
1. **Template library** — verified templates for common patterns
1. **Developer tooling** — debugging, testing, visualization

See <CONTRIBUTING.md> for guidelines.

-----

## FAQ

### How is this different from other L2s?

Most L2s scale Ethereum by providing cheaper EVM execution. FVL specializes Ethereum by replacing EVM execution entirely for a specific class of systems. The tradeoff is deliberate — less expressiveness in exchange for formal guarantees that general-purpose execution cannot provide. Not better or worse than Optimism or Arbitrum. Categorically different in purpose.

### Why does constrained expressiveness matter for security?

Because the attack surface of a system is proportional to its expressive power. A Turing-complete environment can express any computation, including computations that were never intended. FVL’s primitive set can only express what it was designed to express. Vulnerabilities that emerge from arbitrary computation — reentrancy, unbounded loops, unexpected state transitions — are not possible by construction because the constructs that produce them do not exist in the language.

### Why YAML?

Financial systems are patterns. YAML makes those patterns explicit and human-readable. The goal is not convenience — it is to make the full logic of a financial system visible and auditable without requiring code comprehension. A non-technical stakeholder reading a FVL template can verify what the system does. That property is impossible to achieve with Solidity.

### Can I use this in production?

Not yet. The MVP proves the architectural thesis and validates the primitive set. Production use requires fraud proofs, an asset bridge, and a decentralized sequencer. These are on the roadmap.

### How do I propose a new primitive?

Open an issue with the use case, proposed syntax, security considerations, and an argument for why it is a fundamental building block rather than a composition of existing primitives. The standard for inclusion is high by design. Primitive additions expand the attack surface.

-----

## Documentation

- [FVL Language Specification](docs/FVL_SPEC.md) — complete primitive reference
- [REPL User Guide](docs/FVL_REPL_Guide.md) — interactive console tutorial
- [FVL Overview](docs/FVL_PROJECT_OVERVIEW.md) — technical deep dive
- [Template Examples](tests/fixtures/) — example systems

-----

## Community

- **Twitter/X:** [@FVL_Finance](https://twitter.com/fvl_finance)
- **Discord:** [discord.gg/uRrtJQrp](https://discord.gg/uRrtJQrp)

-----

## License

GNU General Public License v3.0. Use, modify, and distribute freely. Derivative works must remain open source under GPLv3. See <LICENSE> for full terms.

-----

## Acknowledgments

- Ethereum Foundation — settlement layer
- Optimism — rollup architecture
- Foundry — development tooling

-----

**FVL: The execution layer for financial systems that don’t require arbitrary computation.**

[Get Started](#quick-start) | [View Examples](tests/fixtures/) | [Read Docs](docs/) | [Join Community](#community)

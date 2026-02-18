# FVL: Financial Value Language

## Executive Summary

FVL (Financial Value Language) is a declarative framework for creating composable financial systems on Ethereum. Unlike traditional smart contract platforms where financial logic is programmed, FVL allows financial systems to be declared through simple, human-readable templates.

**Core Innovation:** Financial systems as reusable building blocks that can be assembled, deployed, and verified without writing code.

**Current State:** Functional MVP with Ethereum settlement, demonstrating the core vision with foundational infrastructure in place.

---

## The Problem

### Traditional DeFi Development is Broken

Building financial systems on Ethereum today requires:

1. **Deep Technical Expertise**
   - Solidity programming knowledge
   - Understanding of EVM internals
   - Security audit expertise
   - Gas optimization skills

2. **High Development Costs**
   - Months of development time
   - Expensive security audits
   - Ongoing maintenance burden
   - Constant vulnerability monitoring

3. **Siloed Innovation**
   - Each protocol reinvents the wheel
   - No standardized building blocks
   - Difficult to compose systems safely
   - Knowledge trapped in code

4. **Centralization of Power**
   - Only well-funded teams can build
   - Users can't create their own systems
   - Financial sovereignty remains out of reach
   - Innovation concentrated in few hands

### The Deeper Issue: Programmability vs. Declarability

The Ethereum ecosystem chose **programmability** (Solidity, EVM) over **declarability**. This made sense for general computation, but for financial systems specifically, it created unnecessary complexity.

Financial systems have predictable patterns:
- Collect assets under certain conditions
- Apply rules and formulas
- Distribute according to logic
- Track state over time

These patterns should be **declared**, not **programmed**.

---

## The FVL Solution

### Financial Systems as Composable Building Blocks

FVL introduces a framework where financial systems are assembled from reusable primitives rather than programmed from scratch. Think of it as **financial Lego blocks** — standardized pieces that snap together in unlimited combinations.

Instead of writing code, you declare what your system does:

```yaml
system: "SimpleStaking"

pool:
  collect:
    from: token_holders(0xTokenAddress)
    what: ERC20(0xStakingToken)
    min: "100"
    cap: "1000000"

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
```

This template:
- Is **human-readable** — anyone can understand what it does
- Is **deterministic** — same template always produces same behavior
- Is **composable** — can reference and build on other systems
- Is **verifiable** — security properties are immediately visible

### The Building Blocks

FVL provides fundamental primitives that can be combined infinitely:

**Access Control Blocks:**
- `anyone` — open to all
- `token_holders(address)` — gated by token ownership
- `whitelist([addresses])` — specific allow list
- `nft_holders(address)` — NFT-based access
- `min_balance(amount, token)` — minimum balance requirement

**Asset Blocks:**
- `ETH` — native Ethereum
- `ERC20(address)` — fungible tokens
- `ERC721(address)` — NFTs
- `ERC1155(address, id)` — semi-fungible tokens
- `multiple([...])` — multi-asset systems

**Condition Blocks:**
- Balance comparisons (`balance > X`)
- Time-based triggers (`time > timestamp`)
- Oracle data (`price(feed) < threshold`)
- Holder counts (`holders >= N`)
- Value tracking (`total_value == cap`)

**Action Blocks:**
- `enable(permission)` — grant access
- `liquidate(target)` — enforce conditions
- `mint(amount, to)` — create assets
- `burn(amount, from)` — destroy assets
- `transfer(amount, from, to)` — move assets
- `execute(function)` — trigger system logic

**Distribution Blocks:**
- `proportional` — distribute by share
- `equal` — split evenly
- `weighted(metric)` — custom weighting
- `tiered(thresholds)` — bracket-based
- `quadratic` — quadratic funding formula

**Time Blocks:**
- `linear(duration)` — linear vesting
- `cliff(duration)` — cliff period
- `graded(schedule)` — milestone-based
- `locks: duration` — time locks

### Unlimited Combinations

These blocks can be combined to create any financial system imaginable:

**Example 1: Community Treasury**
```
collect: token_holders
distribute: quadratic
triggers: governance_vote
time: cliff + linear_vesting
```

**Example 2: Conditional Lending Pool**
```
collect: anyone with min_balance
rules: collateral_ratio < 1.5 → liquidate
distribute: proportional to lenders
oracles: price_feed
```

**Example 3: Tiered Staking**
```
collect: anyone
distribute: tiered by lock_duration
time: multiple cliff periods
rights: time_based permissions
```

**Example 4: Cross-System Yield Aggregator**
```
collect: anyone
rules: balance > threshold → deposit_to(system_id)
distribute: compound_and_redistribute
oracles: external_APY_feeds
```

The limitation is **imagination**, not technical capability.

### Core Value Propositions

#### 1. Democratization Through Building Blocks

Anyone can create financial systems by:
- Selecting the blocks they need
- Arranging them in a template
- Deploying instantly
- No programming required

Financial innovation becomes accessible to all.

#### 2. Safety Through Constraint

The building blocks are carefully designed:
- Finite set of primitives
- No unbounded loops or recursion
- Deterministic state transitions
- Security properties enforced by design

You can't misuse blocks in ways that create vulnerabilities.

#### 3. Composability as Foundation

Systems built from blocks can reference other systems:
- System A can deposit to System B
- System C can query System A's state
- System D can combine A, B, and C
- Infinite depth of composition

Financial Lego blocks that snap together seamlessly.

#### 4. Transparency and Verification

Every system's blocks are visible:
- Template shows all logic
- No hidden behavior
- Deterministic executionQuick
- Anyone can verify

Radical transparency through declarative design.

---

## How It Works

### High-Level Architecture

FVL operates as a specialized execution layer that settles to Ethereum:

1. **Declaration Layer**
   - Users write financial systems in YAML
   - Templates define pools, rules, distributions, time constraints
   - System IDs are computed from template content

2. **Execution Layer**
   - Transactions interact with deployed systems
   - State transitions follow system rules
   - All execution is deterministic and replayable
   - State roots track system state

3. **Settlement Layer (Ethereum)**
   - State roots are anchored on Ethereum
   - Provides data availability
   - Enables final settlement
   - Creates verifiable history

### Key Design Principles

#### Determinism First

Every FVL system is a finite state machine. Given:
- Initial state
- Ordered sequence of transactions

The final state is **always the same**. This enables:
- Independent verification
- Dispute resolution (when fraud proofs are added)
- State reconstruction
- Trust minimization

#### Constraint as Feature

FVL intentionally limits what you can express:
- No loops
- No recursion
- No floating point math
- Fixed set of primitives

These constraints **create safety**. You can't write a reentrancy attack because the language doesn't allow the patterns that enable it.

#### Financial Primitives Only

FVL provides building blocks specifically for financial coordination:

**Access Control:**
- anyone
- token_holders
- whitelist
- nft_holders
- min_balance

**Assets:**
- ETH
- ERC20 tokens
- ERC721 NFTs
- Multiple asset types

**Rules:**
- Conditional logic (if/then)
- Balance checks
- Time-based triggers
- Oracle data feeds
- Price conditions

**Distributions:**
- Proportional
- Equal split
- Weighted by metric
- Tiered rewards
- Quadratic funding

**Time Constraints:**
- Vesting schedules
- Cliff periods
- Lock durations
- Milestone-based release

These primitives cover the vast majority of financial coordination use cases.

---

## Use Cases

### 1. Community Treasury Management

A DAO can deploy a treasury system that:
- Collects contributions from token holders
- Distributes funds based on governance votes
- Implements vesting for team allocations
- Enforces spending limits and conditions

All without writing smart contract code.

### 2. Specialized Staking Mechanisms

Create custom staking systems with:
- Unique reward formulas
- Dynamic APY based on conditions
- Lock periods tied to performance
- Integration with external oracles

### 3. Lending and Borrowing Pools

Deploy lending pools with:
- Custom collateralization ratios
- Oracle-based liquidation triggers
- Interest rate curves
- Risk parameters tuned to specific assets

### 4. Crowdfunding and Token Sales

Launch fundraising campaigns with:
- Minimum/maximum contribution limits
- Refund conditions
- Vesting schedules for raised funds
- Milestone-based releases

### 5. Yield Aggregation

Build systems that:
- Automatically route deposits to highest yield
- Rebalance based on conditions
- Compound rewards
- Distribute profits to liquidity providers

### 6. Insurance Pools

Create mutual insurance with:
- Premium collection mechanisms
- Claim evaluation logic
- Payout formulas
- Reserve requirements

### 7. Prediction Markets

Deploy markets with:
- Custom resolution conditions
- Payout distributions
- Time-based settlement
- Oracle integration

---

## Current State: MVP Capabilities

### What Works Today

#### System Deployment
- Deploy any valid FVL YAML template
- System ID generation (content-addressed)
- State initialization
- Transaction processing

#### Asset Management
- Internal balance tracking
- Transfers between addresses
- Support for ETH and ERC20 (symbolic)
- Multi-asset systems

#### System Interaction
- Condition evaluation
- Action triggering
- Rule execution
- State updates

#### Oracle Integration
- Oracle value updates (deployer-only)
- Condition evaluation based on oracle data
- Support for multiple oracles per system

#### State Management
- Deterministic state computation
- State root generation
- Block production
- Transaction ordering (nonce-based)

#### Ethereum Settlement
- State root submission to L1
- Append-only block log
- Data availability on Ethereum
- Verifiable transaction history

#### Developer Tools
- Interactive REPL for system interaction
- Command-line interface for all operations
- JSON output for automation
- State reconstruction from logs

---
## Current Limitations

### Technical Limitations

#### 1. Trust Assumptions

**Current State:**
- Users must trust the sequencer
- No fraud proof system (yet)
- No permissionless challenge mechanism
- Sequencer can censor transactions

**Impact:**
- Not suitable for adversarial environments
- Requires trust in operator
- Cannot guarantee censorship resistance

**Mitigation Path:**
- Fraud proofs (planned)
- Challenge periods (planned)
- Forced inclusion (planned)

#### 2. Asset Bridging

**Current State:**
- No L1↔L2 token bridge
- Balances are internal only
- Cannot deposit real assets from Ethereum
- Cannot withdraw to Ethereum

**Impact:**
- Testing/demo use only
- No real economic value
- Manual balance initialization required

**Mitigation Path:**
- Bridge contract implementation
- Deposit proofs
- Withdrawal mechanisms
- Merkle proof verification

#### 3. Centralized Sequencing

**Current State:**
- Single sequencer operator
- No decentralized block production
- Single point of failure

**Impact:**
- Liveness depends on sequencer
- Potential for censorship

**Mitigation Path:**
- Decentralized sequencer network
- Leader election protocol

#### 4. Limited Asset Support

**Current State:**
- Symbolic ERC20/ERC721 tracking only
- No actual token movement
- No NFT transfers
- No native DeFi integrations

**Impact:**
- Cannot interact with existing DeFi
- Isolated ecosystem
- No composability with Ethereum protocols

**Mitigation Path:**
- Asset bridge contracts
- Standard token interfaces
- DeFi protocol adapters

### Feature Limitations

#### 1. Expression Language

**Current State:**
- Fixed set of conditions and actions
- No custom formula definitions
- Limited mathematical operations
- No user-defined functions

**Impact:**
- Some financial models cannot be expressed
- May need template updates for new patterns

**Future Direction:**
- Carefully expand primitive set
- Add formula builder
- Maintain safety constraints

#### 2. Oracle Network

**Current State:**
- Deployer-only oracle updates
- No decentralized oracle integration
- Manual price feeds
- No automation

**Impact:**
- Requires trusted oracle operators
- Manual intervention needed
- Potential for stale data

**Future Direction:**
- Chainlink integration
- Decentralized oracle networks
- Automated feeds

#### 3. Governance

**Current State:**
- No built-in governance primitives
- Cannot upgrade deployed systems
- Fixed parameters after deployment

**Impact:**
- Systems are immutable
- Cannot adapt to changing conditions
- Requires redeployment for changes

**Future Direction:**
- Governance templates
- Parameter adjustment mechanisms
- Upgrade patterns

---

## What Makes FVL Different

## FVL's Position in the Ethereum Ecosystem

### Most L2s Focus on Scaling

Traditional Layer 2 solutions (Optimism, Arbitrum, zkSync) focus primarily on:
- Reducing transaction costs
- Increasing throughput
- Providing EVM equivalence
- Scaling general-purpose computation

They replicate Ethereum's capabilities at lower cost, but don't fundamentally change **what** you can build or **how** you build it.

### FVL: Purpose-Built for Financial Systems

FVL represents a different approach to L2s:

**Not just scaling** — FVL is a **specialized execution environment** for financial coordination.

**Not EVM-equivalent** — FVL uses a constrained, domain-specific language designed specifically for financial systems.

**Not general-purpose** — FVL deliberately limits what can be expressed to ensure safety and accessibility.

### Building on Ethereum's Foundation

Ethereum established:
- Decentralized consensus
- Programmable value transfer
- General-purpose computation
- Secure settlement layer

FVL builds on this foundation by adding:
- Financial system primitives
- Declarative templates
- Composable building blocks
- Accessible creation tools

### The Specialization Thesis

Just as Ethereum specialized from Bitcoin (adding programmability), FVL specializes from Ethereum (focusing on financial systems).

- **Bitcoin** → Store of value
- **Ethereum** → General-purpose compute
- **FVL** → Financial system coordination

Each layer adds specialization while inheriting security from the layer below.

---

## What Makes FVL Unique

FVL is purpose-built for financial systems, distinguishing it from general-purpose platforms:

### Financial Building Blocks

FVL provides composable primitives specifically designed for financial coordination. Users combine these blocks to create systems limited only by imagination:

- Access control mechanisms
- Asset collection and distribution
- Time-based constraints
- Conditional logic
- Oracle integration
- Rights management

These primitives can be assembled in countless combinations to create novel financial systems.

### Declarative Over Programmatic

Financial systems are declared through templates, not programmed through code. This approach:
- Makes creation accessible to non-developers
- Enforces safety through constraints
- Enables instant verification
- Allows template reuse and composition

---

## Vision: Where FVL is Heading

### Post-MVP

1. **Fraud Proof System**
   - Trustless verification
   - Challenge mechanisms
   - Slashing conditions
   - Become true Optimistic Rollup

2. **Asset Bridge**
   - L1→L2 deposits
   - L2→L1 withdrawals
   - Real economic activity
   - Merkle proof verification

3. **Template Marketplace**
   - Curated template library
   - Community contributions
   - Template versioning

4. **Enhanced Primitives**
   - More distribution formulas
   - Advanced time mechanics
   - Additional condition types
   - Richer asset support

### Medium Term

1. **Decentralized Sequencing**
   - Multiple sequencer operators
   - Leader election

2. **Cross-System Composability**
   - Systems reference other systems
   - Inter-system transactions
   - Nested compositions

3. **Oracle Network Integration**
   - Chainlink integration
   - Multiple data providers

4. **Governance Primitives**
   - Parameter adjustment
   - System upgrades

---

## Why Now?

### The DeFi Market is Ready

1. **User Sophistication**
   - Millions familiar with DeFi
   - Understanding of financial primitives
   - Demand for customization
   - Appetite for new tools

2. **Infrastructure Maturity**
   - Ethereum stable and secure
   - Rollup technology proven
   - Oracle networks established
   - Developer tooling excellent

3. **Demand for Simplification**
   - Security concerns paramount
   - Audit costs prohibitive
   - Development time too long
   - Need for standardization

4. **Sovereignty Movement**
   - Users want control
   - Distrust of centralized finance
   - Desire for transparency
   - Community ownership values

---

## Conclusion

FVL represents a new category of Layer 2: **purpose-built for financial systems** rather than general-purpose scaling.

While Ethereum provides the foundation for programmable value, FVL specializes that foundation into composable building blocks that anyone can use to create financial systems.

**Core Insight:** Financial systems follow predictable patterns. These patterns should be expressed through composable primitives, not programmed from scratch each time.

**The Innovation:** An Optimistic Rollup designed specifically for assembling financial systems from building blocks.

**Ultimate Goal:** Enable anyone to create sophisticated financial coordination mechanisms by combining simple, safe, verifiable primitives.

**The Opportunity:** Most L2s scale Ethereum. FVL **specializes** Ethereum for the financial systems domain.

---

## Get Involved

FVL is building the future of financial coordination through composable building blocks.

Financial systems are limited only by imagination — not by technical barriers.
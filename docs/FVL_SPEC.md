# FVL Language Specification

**Version:** 1.0.0 
**Status:** Draft  
**Last Updated:** February 2026

---

## Table of Contents

1. [Introduction](#introduction)
2. [Design Philosophy](#design-philosophy)
3. [System Structure](#system-structure)
4. [Type System](#type-system)
5. [Primitives Reference](#primitives-reference)
6. [Composition Rules](#composition-rules)
7. [Security Model](#security-model)
8. [Examples](#examples)

---

## Introduction

FVL (Financial Value Language) is a declarative, domain-specific language for expressing financial coordination systems. It is designed to be:

- **Constrained** — only financial primitives, no general computation
- **Deterministic** — same input always produces same output
- **Composable** — systems can reference other systems
- **Verifiable** — security properties visible in template
- **Accessible** — readable by non-developers

FVL systems are finite state machines deployed to an Optimistic Rollup that settles to Ethereum.

---

## Design Philosophy

### Constraint as Feature

FVL deliberately limits expressiveness to ensure safety:

- **No loops** — prevents infinite execution
- **No recursion** — prevents stack overflow
- **No floating point** — prevents rounding errors (use fixed-point u128)
- **Finite primitives** — attack surface is bounded

These constraints eliminate entire classes of vulnerabilities.

### Declarative Over Imperative

Financial systems are **described**, not **programmed**:

```yaml
# Declarative: what the system does
distribute:
  formula: proportional
  to: contributors
  
# NOT imperative: how to do it
# for each contributor:
#   calculate share
#   transfer amount
```

This makes security properties immediately visible.

### Determinism First

Given:
- Initial state
- Ordered transaction sequence

The final state is **always identical**. This enables:
- Independent verification
- Dispute resolution
- State reconstruction
- Trust minimization

---

## System Structure

Every FVL system has six top-level sections:

```yaml
system: <string>        # System name
pool: <pool_config>     # Asset collection rules
rules: <rules_config>   # Conditional logic and distributions
rights: <rights_map>    # Permission definitions
time: <time_config>     # Temporal constraints
oracles: <oracle_list>  # External data feeds
```

All sections are **required** (though some may be empty).

---

## Type System

### Primitive Types

#### String
UTF-8 strings, max 256 characters.

```yaml
system: "MySystem"
name: "eth_price"
```

#### Integer (u128)
Unsigned 128-bit integers for amounts and values.

```yaml
amount: "1000"
min: "100"
cap: "1000000"
```

#### Timestamp (u64)
Unix timestamps in seconds.

```yaml
timestamp: "1735689600"
deployed_at: "1708355290"
```

#### Address (string)
Ethereum addresses, must match `^0x[a-fA-F0-9]{40}$`

```yaml
address: "0x1234567890123456789012345678901234567890"
```

#### Duration (u64)
Time duration in seconds.

```yaml
seconds: "2592000 "     # 30 days
duration: "7776000 "    # 90 days
```

### Composite Types

#### List
Ordered sequences of elements.

```yaml
addresses: ["0x123...", "0xabc..."]
permissions: [stake, unstake, withdraw]
thresholds: ["1000, 5000, 10000"]
```

#### Map
Key-value pairs (string keys only).

```yaml
rights:
  admin: [pause, unpause]
  user: [stake, unstake]
```

---

## Primitives Reference

### 1. System Declaration

```yaml
system: <string>
```

**Required.** Identifies the system. Used in logs and references.

**Constraints:**
- 1-64 characters
- Alphanumeric and spaces only
- Must be unique within deployment context

**Example:**
```yaml
system: "Community Treasury v2"
```

---

### 2. Pool Configuration

Defines how assets are collected into the system.

```yaml
pool:
  collect:
    from: <access_rule>
    what: <asset_type>
    min: <amount>
    max: <max_amount>
    cap: <cap_amount>
```

#### 2.1 Access Rules

Controls who can contribute to the pool.

##### `anyone`
Open to all addresses.

```yaml
from:
  type: anyone
```

##### `token_holders`
Restricted to holders of a specific ERC20.

```yaml
from:
  type: token_holders
  address: "0xTokenAddress"
```

##### `nft_holders`
Restricted to holders of a specific ERC721.

```yaml
from:
  type: nft_holders
  address: "0xNFTAddress"
```

##### `whitelist`
Explicit list of allowed addresses.

```yaml
from:
  type: whitelist
  addresses:
    - "0xAddress1"
    - "0xAddress2"
    - "0xAddress3"
```

##### `min_balance`
Requires minimum balance of a token.

```yaml
from:
  type: min_balance
  amount: "1000"
  token: "0xTokenAddress"
```

#### 2.2 Asset Types

Defines what can be collected.

##### `eth`
Native Ethereum.

```yaml
what:
  type: eth
```

##### `erc20`
Fungible tokens.

```yaml
what:
  type: erc20
  address: "0xTokenAddress"
```

##### `erc721`
Non-fungible tokens.

```yaml
what:
  type: erc721
  address: "0xNFTAddress"
```

##### `erc1155`
Semi-fungible tokens.

```yaml
what:
  type: erc1155
  address: "0xTokenAddress"
  id: "42"
```

##### `multiple`
Multiple asset types accepted.

```yaml
what:
  type: multiple
  assets:
    - type: eth
    - type: erc20
      address: "0xUSDC"
    - type: erc20
      address: "0xDAI"
```

#### 2.3 Amount Constraints

##### Minimum Amount

```yaml
min:
  type: value
  amount: "100"

# OR
min:
  type: zero
```

##### Maximum Amount

```yaml
max:
  type: value
  amount: "10000"

# OR
max:
  type: none
```

##### Total Cap

```yaml
cap:
  type: value
  amount: "1000000"

# OR
cap:
  type: none
```

---

### 3. Rules Configuration

Defines conditional logic and distributions.

```yaml
rules:
  conditions: <condition_list>
  distribute: <distribution_config>
```

#### 3.1 Conditions

List of if-then rules.

```yaml
conditions:
  - if: <expression>
    then: <action>
  - if: <expression>
    then: <action>
```

#### 3.2 Expressions

##### Balance Comparisons

```yaml
if:
  type: balance_gt
  value: "1000"

if:
  type: balance_lt
  value: "500"

if:
  type: balance_eq
  value: "1000"

if:
  type: balance_gte
  value: "1000"

if:
  type: balance_lte
  value: "500"
```

##### Price Conditions (Oracle-based)

```yaml
if:
  type: price_gt
  oracle: "eth_price"
  value: "3000"

if:
  type: price_lt
  oracle: "eth_price"
  value: "2000"

if:
  type: price_eq
  oracle: "eth_price"
  value: "2500"
```

##### Time Conditions

```yaml
if:
  type: time_gt
  timestamp: "1735689600"

if:
  type: time_lt
  timestamp: "1735689600"
```

##### Holder Counts

```yaml
if:
  type: holders_count_gte
  count: "100"

if:
  type: holders_count_lte
  count: "50"
```

##### Value Tracking

```yaml
if:
  type: total_value_eq
  value: "1000000"
```

##### Collateral Ratio

```yaml
if:
  type: collateral_ratio_lt
  ratio: "150"
```

##### Utilization

```yaml
if:
  type: utilization_gt
  ratio: "90"
```

##### Named Events

```yaml
if:
  type: swap_requested

if:
  type: liquidity_added
```

#### 3.3 Actions

##### `enable`
Grant a permission.

```yaml
then:
  type: enable
  permission: "withdraw"
```

##### `disable`
Revoke a permission.

```yaml
then:
  type: disable
  permission: "stake"
```

##### `liquidate`
Enforce liquidation on target.

```yaml
then:
  type: liquidate
  target: "borrower"
```

##### `mint`
Create assets.

```yaml
then:
  type: mint
  amount: "1000"
  to: "0xRecipient"
```

##### `burn`
Destroy assets.

```yaml
then:
  type: burn
  amount: "500"
  from: "0xHolder"
```

##### `transfer`
Move assets between addresses.

```yaml
then:
  type: transfer
  amount: "100"
  from: "0xSender"
  to: "0xReceiver"
```

##### `pause`
Pause system operations.

```yaml
then:
  type: pause
```

##### `unpause`
Resume system operations.

```yaml
then:
  type: unpause
```

##### `execute`
Trigger named function.

```yaml
then:
  type: execute
  function: "rebalance"
```

##### Named Actions

```yaml
then:
  type: execute_swap

then:
  type: increase_interest_rate
```

#### 3.4 Distribution Configuration

```yaml
distribute:
  formula: <distribution_formula>
  to: <recipient_group>
  triggers: <trigger_type>
```

##### Distribution Formulas

**Proportional**
Distribute by stake size.

```yaml
formula:
  type: proportional
```

**Equal**
Equal split among recipients.

```yaml
formula:
  type: equal
```

**Weighted**
Custom weighting by metric.

```yaml
formula:
  type: weighted
  metric: "contribution_score"
```

**Tiered**
Bracket-based distribution.

```yaml
formula:
  type: tiered
  thresholds: ["1000, 5000, 10000"]
```

**Quadratic**
Quadratic funding formula.

```yaml
formula:
  type: quadratic
```

##### Recipient Groups

**Contributors**
All who contributed to pool.

```yaml
to:
  type: contributors
```

**All Holders**
All token holders.

```yaml
to:
  type: all_holders
```

**Top N**
Top N by some metric.

```yaml
to:
  type: top_n
  count: "10"
```

**Role**
Specific role holders.

```yaml
to:
  type: role
  name: "stakers"
```

**Conditional**
Recipients meeting condition.

```yaml
to:
  type: conditional
  expression:
    type: balance_gt
    value: "1000"
```

##### Triggers

```yaml
triggers: on_swap
triggers: continuous
triggers: manual
```

---

### 4. Rights Management

Permission definitions for roles.

```yaml
rights:
  <role_name>: [<permission>, <permission>, ...]
  <role_name>: [<permission>, ...]
```

**Example:**
```yaml
rights:
  admin: [pause, unpause, set_params]
  staker: [stake, unstake, claim_rewards]
  liquidator: [liquidate]
  anyone: [view]
```

**Constraints:**
- Role names: alphanumeric, max 64 chars
- Permissions: alphanumeric + underscore, max 64 chars
- No reserved role names: `system`, `deployer`

---

### 5. Time Configuration

Temporal constraints on the system.

```yaml
time:
  start: <time_value>
  end: <time_value>
  locks: <lock_value>
  vesting: <vesting_value>
  cliffs: <u64>              # Optional
```

#### 5.1 Time Values

**Now**
Current timestamp at deployment.

```yaml
start:
  type: now
```

**Specific Timestamp**
Unix timestamp.

```yaml
start:
  type: timestamp
  value: "1735689600"
```

**None**
No time constraint.

```yaml
end:
  type: none
```

#### 5.2 Lock Values

**Duration**
Time lock in seconds.

```yaml
locks:
  type: duration
  seconds: "2592000 " # 30 days
```

**None**
No lock.

```yaml
locks:
  type: none
```

#### 5.3 Vesting Values

**Linear**
Linear vesting over duration.

```yaml
vesting:
  type: linear
  duration: "7776000 " # 90 days
```

**Cliff**
Cliff period before vesting.

```yaml
vesting:
  type: cliff
  duration: "2592000 " # 30 days
```

**Graded**
Milestone-based vesting.

```yaml
vesting:
  type: graded
  schedule: ["2592000, 5184000, 7776000"]
```

**Milestone**
Condition-based vesting.

```yaml
vesting:
  type: milestone
  conditions: ["first_goal", "second_goal", "third_goal"]
```

**None**
No vesting.

```yaml
vesting:
  type: none
```

#### 5.4 Cliff Period

Optional additional cliff (in seconds).

```yaml
cliffs: "604800 " # 7 days
```

---

### 6. Oracle Configuration

External data feeds.

```yaml
oracles:
  - name: <string>
    type: <string>
    source: <string>
  - name: <string>
    type: <string>
    source: <string>
```

**Example:**
```yaml
oracles:
  - name: "eth_price"
    type: "price_feed"
    source: "chainlink/ETH-USD"
  - name: "btc_price"
    type: "price_feed"
    source: "chainlink/BTC-USD"
  - name: "tvl_metric"
    type: "custom"
    source: "internal/tvl"
```

**Constraints:**
- Name: unique within system, alphanumeric + underscore
- Type: arbitrary string (for now)
- Source: arbitrary string (for now)

**Note:** Oracle integration is manual in MVP. Automated feeds coming post-MVP.

---

## Composition Rules

### System References

Systems can reference other deployed systems (post-MVP):

```yaml
rules:
  conditions:
    - if:
        type: balance_gt
        value: "10000"
      then:
        type: deposit_to_system
        system_id: "0xOtherSystemID"
```

### Constraints

1. **No circular references** — System A cannot reference System B if B references A
2. **Type safety** — Assets must match across system boundaries

---

## Security Model

### Threat Model

FVL protects against:
- **Reentrancy** — language doesn't allow it
- **Integer overflow** — u128 with explicit bounds
- **Unbounded loops** — no loops exist
- **Recursion attacks** — no recursion allowed
- **Oracle manipulation** — values are explicit, tampering is visible

FVL does NOT protect against:
- **Sequencer censorship** — trust assumption (for now)
- **Oracle data quality** — garbage in, garbage out
- **Economic attacks** — game theory is the user's responsibility
- **Governance failures** — social layer issues

### Determinism Guarantees

Given:
```
S0 = initial state
T = [tx1, tx2, tx3, ..., txN]
```

Then:
```
apply(S0, T) → SN
```

Always produces the same `SN`, regardless of:
- Which node computes it
- When it's computed
- How many times it's computed

This enables trustless verification.

### Auditability

Every system:
- Has a content-addressed ID (hash of template)
- Publishes full transaction data on-chain
- Produces verifiable state transitions
- Can be replayed independently

Security properties are **visible in the template**, not hidden in bytecode.

---

## Examples

### Example 1: Simple Staking

```yaml
system: "SimpleStaking"

pool:
  collect:
    from:
      type: anyone
    what:
      type: erc20
      address: "0xStakingToken"
    min:
      type: value
      amount: "100"
    max:
      type: none
    cap:
      type: none

rules:
  conditions:
    - if:
        type: time_gt
        timestamp: "1735689600"
      then:
        type: enable
        permission: unstake
  
  distribute:
    formula:
      type: proportional
    to:
      type: contributors
    triggers: continuous

rights:
  anyone: [stake, view]
  contributors: [unstake, claim_rewards]

time:
  start:
    type: now
  end:
    type: none
  locks:
    type: duration
    seconds: "2592000"
  vesting:
    type: none

oracles: []
```

### Example 2: Lending Pool

```yaml
system: "LendingPool"

pool:
  collect:
    from:
      type: anyone
    what:
      type: erc20
      address: "0xUSDC"
    min:
      type: value
      amount: "100"
    max:
      type: none
    cap:
      type: value
      amount: "10000000"

rules:
  conditions:
    - if:
        type: collateral_ratio_lt
        ratio: "150"
      then:
        type: liquidate
        target: "borrower"
    
    - if:
        type: utilization_gt
        ratio: "90"
      then:
        type: increase_interest_rate
  
  distribute:
    formula:
      type: proportional
    to:
      type: contributors
    triggers: continuous

rights:
  anyone: [view]
  contributors: [deposit, withdraw]
  borrowers: [borrow, repay]
  liquidators: [liquidate]

time:
  start:
    type: now
  end:
    type: none
  locks:
    type: none
  vesting:
    type: none

oracles:
  - name: "eth_price"
    type: "price_feed"
    source: "chainlink/ETH-USD"
  - name: "utilization"
    type: "internal"
    source: "pool/utilization_rate"
```

### Example 3: Tiered Crowdfunding

```yaml
system: "TieredCrowdfund"

pool:
  collect:
    from:
      type: anyone
    what:
      type: eth
    min:
      type: value
      amount: "10"
    max:
      type: value
      amount: "10000"
    cap:
      type: value
      amount: "1000000"

rules:
  conditions:
    - if:
        type: total_value_eq
        value: "1000000"
      then:
        type: enable
        permission: claim_tokens
    
    - if:
        type: time_gt
        timestamp: "1740960000"
      then:
        type: enable
        permission: refund
  
  distribute:
    formula:
      type: tiered
      thresholds: ["100, 1000, 10000"]
    to:
      type: contributors
    triggers: manual

rights:
  anyone: [contribute]
  contributors: [claim_tokens, refund]
  admin: [finalize, emergency_stop]

time:
  start:
    type: now
  end:
    type: timestamp
    value: "1740960000"
  locks:
    type: duration
    seconds: "7776000"
  vesting:
    type: linear
    duration: "15552000"
  cliffs: "2592000"

oracles: []
```

### Example 4: DAO Treasury

```yaml
system: "DAOTreasury"

pool:
  collect:
    from:
      type: token_holders
      address: "0xDAOToken"
    what:
      type: multiple
      assets:
        - type: eth
        - type: erc20
          address: "0xUSDC"
        - type: erc20
          address: "0xDAI"
    min:
      type: zero
    max:
      type: none
    cap:
      type: none

rules:
  conditions:
    - if:
        type: balance_gt
        value: "100000"
      then:
        type: execute
        function: "invest_surplus"
    
    - if:
        type: holders_count_gte
        count: "100"
      then:
        type: enable
        permission: propose_spending
  
  distribute:
    formula:
      type: quadratic
    to:
      type: conditional
      expression:
        type: balance_gte
        value: "1000"
    triggers: manual

rights:
  token_holders: [contribute, vote, view]
  council: [propose_spending, execute_proposal]
  admin: [pause, unpause, emergency_withdraw]

time:
  start:
    type: now
  end:
    type: none
  locks:
    type: none
  vesting:
    type: graded
    schedule: ["2592000, 7776000, 15552000"]

oracles:
  - name: "treasury_value_usd"
    type: "aggregated"
    source: "internal/total_value"
```

---

## Validation Rules

When parsing FVL templates, the following must be validated:

### Syntax Validation
- Valid YAML structure
- All required top-level keys present
- Correct nesting of objects
- Type constraints respected

### Semantic Validation
- Ethereum addresses match `^0x[a-fA-F0-9]{40}$`
- Amounts are non-negative u128
- Timestamps are valid u64
- Oracle names referenced in conditions exist in oracle list
- Permission names referenced in actions exist in rights map
- Role names are unique
- No circular system references (post-MVP)

### Security Validation
- No expressions create unbounded computation
- Time constraints are logically consistent
- Distribution formulas are well-defined
- Access rules are not contradictory

---

## Future Extensions (Post-MVP)

The following primitives are planned but not yet implemented:

### Cross-System Composition
```yaml
then:
  type: deposit_to_system
  system_id: "0xSystemID"
  amount: "1000"
```

### Advanced Oracle Types
```yaml
oracles:
  - name: "chainlink_vrf"
    type: "randomness"
    source: "chainlink/VRF"
```

### Governance Primitives
```yaml
rules:
  governance:
    voting:
      type: token_weighted
      quorum: "0.1"
      threshold: "0.5"
```

### Dynamic Parameters
```yaml
parameters:
  interest_rate:
    type: dynamic
    formula: "utilization * 0.05"
```

---

## Appendix: Reserved Keywords

The following are reserved and cannot be used as identifiers:

```
system, pool, collect, from, what, min, max, cap
rules, conditions, if, then, distribute, formula, to, triggers
rights, time, start, end, locks, vesting, cliffs, oracles
type, name, source, value, amount, address, timestamp
anyone, token_holders, nft_holders, whitelist, min_balance
eth, erc20, erc721, erc1155, multiple
balance_gt, balance_lt, price_gt, price_lt, time_gt
enable, disable, liquidate, mint, burn, transfer, pause
proportional, equal, weighted, tiered, quadratic
contributors, all_holders, top_n, role, conditional
now, none, duration, linear, cliff, graded, milestone
```

---

## Version History

- **1.0.0** (February 2026) — Initial specification for release

---

**End of Specification**
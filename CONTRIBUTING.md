# Contributing to FVL

Thank you for your interest in contributing to FVL (Financial Value Language). We welcome contributions from the community to help build a comprehensive library of financial system templates and expand the primitive set.

**Note:** This is an MVP. Code contributions to the core implementation are currently out of scope. This guide focuses on template creation and primitive proposals.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Template Contributions](#template-contributions)
3. [Proposing New Primitives](#proposing-new-primitives)
4. [Community](#community)

---

## Code of Conduct

### Our Commitment

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of:
- Experience level
- Technical background
- Identity or background
- Geographic location

### Expected Behavior

- Be respectful and constructive in discussions
- Welcome newcomers and help them get started
- Focus on what is best for the community and project
- Show empathy toward other community members
- Give and accept constructive feedback gracefully

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Personal attacks or trolling
- Publishing others' private information
- Any conduct that would be inappropriate in a professional setting

**Violations:** Report to the maintainers. Violators may be temporarily or permanently banned.

---

## Template Contributions

### What Are Templates?

Templates are pre-built FVL systems that solve common financial coordination problems. They serve as:
- Starting points for new users
- Reference implementations
- Educational examples
- Verified patterns

### Template Categories Needed

We're actively seeking templates in these categories:

**Staking & Rewards**
- Simple token staking
- Tiered staking with multipliers
- Dual-token staking
- NFT staking
- Liquidity mining

**Vesting & Unlocking**
- Linear vesting schedules
- Cliff-based vesting
- Milestone-based unlocking
- Graded release schedules
- Performance-based vesting

**Crowdfunding**
- All-or-nothing campaigns
- Flexible funding
- Tiered rewards
- Refundable contributions
- Quadratic funding

**DAO Treasuries**
- Simple multi-sig treasury
- Token-weighted spending
- Budget allocations
- Proposal-based disbursement
- Emergency fund reserves

**Lending & Borrowing**
- Peer-to-pool lending
- Collateralized borrowing
- Interest rate models
- Liquidation mechanisms

**Insurance & Risk**
- Mutual insurance pools
- Parametric insurance
- Claim processing
- Premium collection
- Reserve management

**Prediction Markets**
- Binary outcome markets
- Multi-outcome markets
- Conditional markets

### Template Quality Standards

All submitted templates must meet these criteria:

#### 1. Functionality
- Template deploys without errors
- Clear purpose and use case
- Parameters are well-documented
- Works as described

#### 2. Documentation
- Clear description of purpose
- Use case explanation
- Parameter documentation
- Example configuration
- Security considerations

#### 3. Code Quality
- Well-formatted YAML (2-space indentation)
- Descriptive comments
- Logical structure
- No hardcoded test values in main template

### Template Submission Process

#### Step 1: Development Setup

```bash
# Clone the repository
git clone https://github.com/fvl-lang/fvl.git
cd fvl

# Build the project
cargo build

# Start local infrastructure
# Terminal 1
anvil

# Terminal 2
bash contracts/deploy.sh

# Terminal 3
cargo run --bin fvl
```

#### Step 2: Create Your Template

Create a new YAML file in `templates/` directory:

```
templates/
  staking/
    simple_staking.yaml
    tiered_staking.yaml
  vesting/
    linear_vesting.yaml
    cliff_vesting.yaml
  crowdfunding/
    all_or_nothing.yaml
  treasury/
    basic_treasury.yaml
```

**Template file structure:**

```yaml
# [Template Name]
# Brief description of what this template does
#
# Use Case: [Specific use case]
# Author: [Your name/handle]
# Date: [YYYY-MM-DD]
#
# Parameters to configure:
# - address fields (replace 0x... placeholders)
# - amounts (adjust to your token decimals)
# - timestamps (set based on your timeline)
# - thresholds (customize to your needs)

system: "TemplateName"

pool:
  collect:
    from:
      type: anyone
    what:
      type: eth
    min:
      type: value
      amount: "100"  # Minimum contribution
    max:
      type: none
    cap:
      type: value
      amount: "1000000"  # Total pool cap

rules:
  conditions:
    - if:
        type: time_gt
        timestamp: "1735689600"  # Replace with your start time
      then:
        type: enable
        permission: withdraw
  
  distribute:
    formula:
      type: proportional
    to:
      type: contributors
    triggers: continuous

rights:
  anyone: [contribute, view]
  contributors: [withdraw, claim]
  admin: [pause, emergency_stop]

time:
  start:
    type: now
  end:
    type: none
  locks:
    type: duration
    seconds: "2592000"  # 30 days
  vesting:
    type: linear
    duration: "7776000"  # 90 days

oracles: []
```

#### Step 3: Document Your Template

Create a corresponding README in the same directory:

**`templates/staking/simple_staking.md`:**

```markdown
# Simple Staking

## Description
A basic token staking system where users lock tokens for a fixed period and earn proportional rewards.

## Use Cases
- Community token staking
- Governance participation incentives
- Liquidity provider rewards

## Parameters

### Required Configuration
- `pool.collect.what.address` — Token to stake (ERC20 address)
- `pool.collect.min.amount` — Minimum stake amount
- `pool.cap.amount` — Maximum total staked
- `time.locks.seconds` — Lock duration
- `time.vesting.duration` — Reward vesting period

### Example Values
- Min stake: 100 tokens
- Max pool: 1,000,000 tokens
- Lock: 30 days
- Vesting: 90 days

## How to Use

1. Deploy the template:
   ```bash
   > deploy templates/staking/simple_staking.yaml
   ```

2. Configure addresses and amounts in YAML before deploying

3. Interact with the system as needed

## Security Considerations

- Lock period prevents early withdrawal
- Cap prevents excessive concentration
- Proportional distribution ensures fairness
- Time-based unlocking is deterministic

## Limitations

- Fixed lock period (no flexible unlocking)
- Single asset staking only
- No slashing mechanism
- No delegation support

## Variations

See also:
- `tiered_staking.yaml` — Multiple lock periods with different rewards
- `nft_staking.yaml` — Stake NFTs instead of tokens


#### Step 4: Submit Pull Request

1. Fork the repository on GitHub
2. Create a branch: `git checkout -b template/your-template-name`
3. Add your template files:
   ```
   templates/category/your_template.yaml
   templates/category/your_template.md
   ```
4. Commit: `git commit -m "Add template: [name]"`
5. Push: `git push origin template/your-template-name`
6. Open Pull Request on GitHub

**PR Description should include:**
- Template name and category
- Use case summary
- Any known limitations
- Link to template documentation

---

## Proposing New Primitives

### What Are Primitives?

Primitives are the building blocks of FVL systems:
- Access control types (`anyone`, `token_holders`)
- Conditions (`balance_gt`, `time_gt`)
- Actions (`mint`, `burn`, `transfer`)
- Distribution formulas (`proportional`, `tiered`)
- Time mechanics (`linear`, `cliff`)

New primitives expand what FVL can express.

### When to Propose a Primitive

Propose a new primitive when:
- **Multiple use cases need it** — solves a recurring pattern
- **Current primitives can't express it** — no composition achieves it
- **It's fundamental** — not a special case or niche need
- **It's safe** — doesn't introduce unbounded computation or security risks

### When NOT to Propose a Primitive

Don't propose if:
- **Existing primitives work** — can be composed from what exists
- **It's too specific** — only one use case
- **It's unbounded** — loops, recursion, or unpredictable computation
- **It's unsafe** — enables attacks or vulnerabilities

### Primitive Proposal Process

#### Step 1: Validate the Need

Answer these questions:

**What can't be built?**
- List specific financial systems
- Show why current primitives fail
- Demonstrate the gap

**Can it be composed?**
- Try building it with existing primitives
- Document why composition doesn't work
- Explain the limitation

**Is it fundamental?**
- Does it enable a category of systems?
- Is it a building block or a complete feature?
- Would other primitives build on it?

**How common is the need?**
- How many use cases does it solve?
- What percentage of financial systems need it?
- Is there community demand?

#### Step 2: Design the Primitive

**Syntax Design**

Be consistent with existing primitives:

```yaml
# Good: follows existing patterns
if:
  type: balance_between
  min: "1000"
  max: "10000"

# Bad: inconsistent syntax
if:
  type: balance_range
  range: ["1000, 10000"]
```

**Semantics**

Define behavior precisely:
- What inputs does it accept?
- What does it evaluate/execute?
- What are the edge cases?
- How does it fail?

**Example:**

```yaml
# New primitive: balance_between
if:
  type: balance_between
  min: "1000"          # Inclusive lower bound
  max: "10000"         # Inclusive upper bound
  asset:             # Optional, defaults to system's primary asset
    type: erc20
    address: "0x..."
```

**Behavior:**
- Returns true if `min <= balance <= max`
- Returns false otherwise
- Works with any asset type
- Min must be less than max (validation error if not)

#### Step 3: Document Security

**Consider:**

**Attack Vectors**
- Can this be manipulated?
- Are there race conditions?
- Can it be front-run?
- Does it enable griefing?

**Resource Constraints**
- Is computation bounded?
- Can it cause DoS?

**Economic Implications**
- Can it be gamed?
- Are incentives aligned?
- What are failure modes?

**Edge Cases**
- What happens with zero values?
- What about maximum values?
- Time-based interactions?
- Oracle dependencies?

#### Step 4: Provide Examples

Show multiple use cases:

**Example 1: Tiered Access**
```yaml
rules:
  conditions:
    - if:
        type: balance_between
        min: "1000"
        max: "5000"
      then:
        type: enable
        permission: tier_1_access
    
    - if:
        type: balance_between
        min: "5001"
        max: "10000"
      then:
        type: enable
        permission: tier_2_access
```

**Example 2: Risk Management**
```yaml
rules:
  conditions:
    - if:
        type: balance_between
        min: "0"
        max: "1000000"
      then:
        type: execute
        function: low_risk_strategy
    
    - if:
        type: balance_between
        min: "1000001"
        max: "10000000"
      then:
        type: execute
        function: high_risk_strategy
```

#### Step 5: Submit Proposal

**Create GitHub Issue:**

Title: `[Primitive Proposal] balance_between condition`

Body:
```markdown
## Primitive Proposal: balance_between

### Problem Statement

Many financial systems need tiered access or behavior based on balance ranges. Current primitives only support single-threshold comparisons (`balance_gt`, `balance_lt`), requiring multiple conditions to check ranges.

**Systems that need this:**
1. Tiered staking (different APY by stake size)
2. Risk-adjusted lending (limits based on total borrowed)
3. Graduated fee structures
4. Membership tiers

**Why composition doesn't work:**
You can combine `balance_gt` and `balance_lt`, but it's verbose and error-prone:

```yaml
# Current workaround (verbose)
conditions:
  - if:
      type: balance_gt
      value: "999"
    then:
      type: enable
      permission: check_upper_bound
  
  - if:
      type: balance_lt
      value: "10001"
    then:
      type: enable
      permission: tier_access
```

This creates two separate conditions that must both be true, complicating logic.

### Proposed Primitive

**Type:** Condition (expression)

**Name:** `balance_between`

**Syntax:**
```yaml
if:
  type: balance_between
  min: <u128>           # Inclusive lower bound
  max: <u128>           # Inclusive upper bound
  asset: <asset_type>   # Optional, defaults to system asset
```

**Semantics:**
- Evaluates to `true` if `min <= current_balance <= max`
- Evaluates to `false` otherwise
- Both bounds are inclusive
- Min must be <= max (validation error if violated)
- Works with all asset types

### Use Cases

**1. Tiered Staking Rewards**
```yaml
rules:
  conditions:
    - if:
        type: balance_between
        min: "100"
        max: "1000"
      then:
        type: set_rate
        value: "5"  # 5% APY

    - if:
        type: balance_between
        min: "1001"
        max: "10000"
      then:
        type: set_rate
        value: "8"  # 8% APY
```

**2. Risk-Based Lending Limits**
```yaml
rules:
  conditions:
    - if:
        type: balance_between
        min: "0"
        max: "100000"
      then:
        type: set_collateral_ratio
        value: "200"  # 200% collateral required

    - if:
        type: balance_between
        min: "100001"
        max: "1000000"
      then:
        type: set_collateral_ratio
        value: "150"  # 150% for larger positions
```

**3. Progressive Fee Structure**
```yaml
rules:
  conditions:
    - if:
        type: balance_between
        min: "0"
        max: "10000"
      then:
        type: set_fee_rate
        value: "100"  # 1% fee

    - if:
        type: balance_between
        min: "10001"
        max: "100000"
      then:
        type: set_fee_rate
        value: "50"  # 0.5% fee
```

### Security Considerations

**Bounded Computation:**
- Single comparison operation
- No loops or recursion
- O(1) complexity

**Attack Vectors:**
- Balance can be manipulated by deposits/withdrawals
- Users can game tier boundaries by adjusting balance
- Mitigation: Use time-locks or minimum hold periods

**Edge Cases:**
- `min == max`: Valid, creates exact balance check
- `min > max`: Validation error at parse time
- Zero values: Valid for lower bound
- Maximum u128: Valid for upper bound

**Economic Considerations:**
- Users will optimize for tier boundaries
- Requires careful threshold selection
- Consider hysteresis or smoothing mechanisms in template design

### Alternatives Considered

**Alternative 1: Keep using balance_gt + balance_lt**
- Rejected: Too verbose, error-prone, less readable

**Alternative 2: Add balance_in_range with array**
```yaml
if:
  type: balance_in_range
  range: ["1000, 10000"]
```
- Rejected: Inconsistent with existing syntax patterns (we use min/max elsewhere)

**Alternative 3: Tiered primitive**
```yaml
if:
  type: in_tier
  tiers: [["0, 1000"], ["1001, 5000"]]
```
- Rejected: Too specific, tries to solve the entire tiering problem rather than providing a building block

#### Step 6: Community Discussion

- Maintainers will review
- Community provides feedback
- Discuss trade-offs
- Refine proposal
- Reach consensus

**Approval criteria:**
- Clear need validated
- Multiple use cases
- Security considered
- Syntax consistent
- Community support

---

## Community

### Communication Channels

- **GitHub Issues:** Primitive proposals
- **GitHub Discussions:** Design discussions, questions, ideas
- **Discord:** [discord.gg/uRrtJQrp](https://discord.gg/uRrtJQrp) — Real-time chat, community support
- **Twitter/X:** [@FVL_Finance](https://twitter.com/fvl_finance) — Announcements, updates

### Getting Help

**Need help contributing?**

1. Check existing documentation
2. Search closed issues
3. Ask in Discord (#contributors-channel)
4. Open a GitHub discussion

---

## Quick Start Guide

Ready to contribute? Here's the fastest path:

### For Template Creators

```bash
# 1. Set up environment
git clone https://github.com/fvl-lang/fvl.git
cd fvl
cargo build

# 2. Start infrastructure
# Terminal 1: anvil
# Terminal 2: bash contracts/deploy.sh
# Terminal 3: cargo run --bin fvl

# 3. Create template
# Edit templates/category/your_template.yaml
# Write templates/category/your_template.md

# 4. Submit
# Create PR with your template files
```

### For Primitive Proposers

```bash
# 1. Validate need
# Try building with existing primitives
# Document why it doesn't work

# 2. Design primitive
# Write clear syntax
# Consider security

# 3. Provide examples
# Multiple use cases
# Real-world scenarios
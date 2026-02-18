use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FvlSystem {
    pub system: String,
    pub pool: Pool,
    pub rules: Rules,
    pub rights: HashMap<String, Vec<String>>,
    pub time: Time,
    pub oracles: Vec<Oracle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pool {
    pub collect: Collect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collect {
    pub from: AccessRule,
    pub what: AssetType,
    pub min: Amount,
    pub max: MaxAmount,
    pub cap: CapAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum AccessRule {
    Anyone,
    TokenHolders {
        address: String,
    },
    Whitelist {
        addresses: Vec<String>,
    },
    NftHolders {
        address: String,
    },
    MinBalance {
        #[serde(with = "u128_as_string")]
        amount: u128,
        token: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum AssetType {
    Eth,
    Erc20 { address: String },
    Erc721 { address: String },
    Erc1155 {
        address: String,
        #[serde(with = "u128_as_string")]
        id: u128,
    },
    Multiple { assets: Vec<AssetType> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Amount {
    Value {
        #[serde(with = "u128_as_string")]
        amount: u128,
    },
    Zero,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MaxAmount {
    None,
    Value {
        #[serde(with = "u128_as_string")]
        amount: u128,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CapAmount {
    None,
    Value {
        #[serde(with = "u128_as_string")]
        amount: u128,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rules {
    pub conditions: Vec<Condition>,
    pub distribute: Distribute,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    #[serde(rename = "if")]
    pub if_expr: Expression,
    pub then: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Expression {
    BalanceGt {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    BalanceLt {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    BalanceEq {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    BalanceGte {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    BalanceLte {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    PriceLt {
        oracle: String,
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    PriceGt {
        oracle: String,
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    PriceEq {
        oracle: String,
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    TimeGt {
        timestamp: u64,
    },
    TimeLt {
        timestamp: u64,
    },
    HoldersCountGte {
        count: u64,
    },
    HoldersCountLte {
        count: u64,
    },
    TotalValueEq {
        #[serde(with = "u128_as_string")]
        value: u128,
    },
    CollateralRatioLt {
        #[serde(with = "u128_as_string")]
        ratio: u128,
    },
    UtilizationGt {
        #[serde(with = "u128_as_string")]
        ratio: u128,
    },
    SwapRequested,
    LiquidityAdded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Action {
    Enable { permission: String },
    Disable { permission: String },
    Liquidate { target: String },
    Mint {
        #[serde(with = "u128_as_string")]
        amount: u128,
        to: String,
    },
    Burn {
        #[serde(with = "u128_as_string")]
        amount: u128,
        from: String,
    },
    Transfer {
        #[serde(with = "u128_as_string")]
        amount: u128,
        from: String,
        to: String,
    },
    Pause,
    Unpause,
    Execute { function: String },
    ExecuteSwap,
    IncreaseInterestRate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Distribute {
    pub formula: DistributionType,
    pub to: RecipientGroup,
    pub triggers: Trigger,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum DistributionType {
    Proportional,
    Equal,
    Weighted { metric: String },
    Tiered {
        thresholds: Vec<u128>,
    },
    Quadratic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RecipientGroup {
    Contributors,
    AllHolders,
    TopN { count: u64 },
    Role { name: String },
    Conditional { expression: Box<Expression> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    OnSwap,
    Continuous,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Time {
    pub start: TimeValue,
    pub end: TimeValue,
    pub locks: LockValue,
    pub vesting: VestingValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cliffs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TimeValue {
    Now,
    Timestamp { value: u64 },
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LockValue {
    None,
    Duration { seconds: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum VestingValue {
    None,
    Linear { duration: u64 },
    Cliff { duration: u64 },
    Graded { schedule: Vec<u64> },
    Milestone { conditions: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Oracle {
    pub name: String,
    #[serde(rename = "type")]
    pub oracle_type: String,
    pub source: String,
}

/// Serde helper: serialize u128 as string for JSON compatibility
pub mod u128_as_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u128>().map_err(serde::de::Error::custom)
    }
}
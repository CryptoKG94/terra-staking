use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub luna_token: String, // luna staking and reward token
    pub ust_token: String,  // ust staking and reward token
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveLuna(Cw20ReceiveMsg),
    ReceiveUst(Cw20ReceiveMsg),
    UnbondLuna {
        amount: Uint128,
    },
    UnbondUst {
        amount: Uint128,
    },
    /// Withdraw pending rewards
    WithdrawLuna {},
    WithdrawUst {},
    /// Owner operation to stop distribution on current staking contract
    /// and send remaining tokens to the new contract
    MigrateStaking {
        new_staking_contract: String,
    },
    UpdateConfig {
        distribution_schedule: Vec<(u64, u64, Uint128)>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Bond {},
}

/// migrate struct for distribution schedule
/// block-based schedule to a time-based schedule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {
        block_time: Option<u64>,
    },
    StakerInfo {
        staker: String,
        block_time: Option<u64>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub luna_token: String,
    pub ust_token: String,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub last_distributed_luna: u64,
    pub total_bond_amount_luna: Uint128,
    pub global_reward_index_luna: Decimal,
    pub last_distributed_ust: u64,
    pub total_bond_amount_ust: Uint128,
    pub global_reward_index_ust: Decimal,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    pub staker: String,
    pub reward_index_luna: Decimal,
    pub bond_amount_luna: Uint128,
    pub pending_reward_luna: Uint128,
    pub reward_index_ust: Decimal,
    pub bond_amount_ust: Uint128,
    pub pending_reward_ust: Uint128,
}

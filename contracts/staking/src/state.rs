use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket};

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";

static PREFIX_REWARD: &[u8] = b"reward";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub luna_token: CanonicalAddr,
    pub ust_token: CanonicalAddr,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub last_distributed_luna: u64,
    pub total_bond_amount_luna: Uint128,
    pub global_reward_index_luna: Decimal,
    pub last_distributed_ust: u64,
    pub total_bond_amount_ust: Uint128,
    pub global_reward_index_ust: Decimal,
}

pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton(storage, KEY_STATE).save(state)
}

pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
    singleton_read(storage, KEY_STATE).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
    pub reward_index_luna: Decimal,
    pub bond_amount_luna: Uint128,
    pub pending_reward_luna: Uint128,
    pub reward_index_ust: Decimal,
    pub bond_amount_ust: Uint128,
    pub pending_reward_ust: Uint128,
}

/// returns return staker_info of the given owner
pub fn store_staker_info(
    storage: &mut dyn Storage,
    owner: &CanonicalAddr,
    staker_info: &StakerInfo,
) -> StdResult<()> {
    Bucket::new(storage, PREFIX_REWARD).save(owner.as_slice(), staker_info)
}

/// remove staker_info of the given owner
pub fn remove_staker_info(storage: &mut dyn Storage, owner: &CanonicalAddr) {
    Bucket::<StakerInfo>::new(storage, PREFIX_REWARD).remove(owner.as_slice())
}

/// returns rewards owned by this owner
/// (read-only version for queries)
pub fn read_staker_info(storage: &dyn Storage, owner: &CanonicalAddr) -> StdResult<StakerInfo> {
    match ReadonlyBucket::new(storage, PREFIX_REWARD).may_load(owner.as_slice())? {
        Some(staker_info) => Ok(staker_info),
        None => Ok(StakerInfo {
            reward_index_luna: Decimal::zero(),
            bond_amount_luna: Uint128::zero(),
            pending_reward_luna: Uint128::zero(),
            reward_index_ust: Decimal::zero(),
            bond_amount_ust: Uint128::zero(),
            pending_reward_ust: Uint128::zero(),
        }),
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128, Coin};

#[cw_serde]
pub struct PoolAsset {
    pub asset: Coin,
    pub weight: Uint128,
}

#[cw_serde]
pub struct PoolParams {
    pub swap_fee: Decimal,
    pub exit_fee: Decimal,
    pub use_oracle: bool,
    pub weight_breaking_fee_multiplier: Decimal,
    pub external_liquidity_ratio: Decimal,
    pub lp_fee_portion: Decimal,
    pub staking_fee_portion: Decimal,
    pub weight_recovery_fee_portion: Decimal,
    pub threshold_weight_difference: Decimal,
    pub fee_denom: String,
}

#[cw_serde]
pub struct Pool {
    pub pool_id: u64,
    pub address: String,
    pub pool_params: PoolParams,
    pub total_shares: Coin,
    pub pool_assets: Vec<PoolAsset>,
    pub total_weight: Uint128,
    pub rebalance_treasury: String,
}

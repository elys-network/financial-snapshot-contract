use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct Apr {
    pub uusdc: u64,
    pub ueden: u64,
    pub uedenb: u64,
}

#[cw_serde]
pub struct BalanceDollar {
    pub asset: String,
    pub amount: u64,
    pub amount_in_usd: Decimal,
}

#[cw_serde]
pub struct VestingDetail {
    pub total_vest: BalanceDollar,
    pub balance_vested: BalanceDollar,
    pub remaining_vest: BalanceDollar,
    pub remaining_time: u64,
}

#[cw_serde]
pub struct EarnDetail {
    pub apr: Apr,
    pub bonding_period: u64,
    pub available: Vec<BalanceDollar>,
    pub staked: Vec<BalanceDollar>,
    pub vesting: Vec<BalanceDollar>,
    pub rewards: Vec<BalanceDollar>,
    pub vesting_details: Vec<VestingDetail>,
    pub asset: String,
}
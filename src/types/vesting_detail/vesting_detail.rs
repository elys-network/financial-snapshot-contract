use crate::types::balance_dollar::balance_dollar::BalanceDollar;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct VestingDetail {
    pub total_vest: BalanceDollar,
    pub balance_vested: BalanceDollar,
    pub remaining_vest: BalanceDollar,
    pub remaining_time: u64,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Int128};
use crate::types::ValidatorDetail;

#[cw_serde]
pub struct QueryBalanceResponse {
    pub balance: Coin,
}

#[cw_serde]
pub struct Delegation {
	pub delegator_address: String,
    pub validator_address: String,
    pub shares: Decimal,
}

#[cw_serde]
pub struct DelegationResponse {
	pub delegation: Delegation,
    pub balance: Coin,
}

#[cw_serde]
pub struct QueryDelegatorDelegationsResponse {
    pub delegation_responses: Vec<DelegationResponse>,
}

#[cw_serde]
pub struct UnbondingDelegationEntry {
    pub balance: Int128,
    pub completion_time: i64,
    pub creation_height: i64,
    pub initial_balance: Int128,
    pub unbonding_id: u64,
}

#[cw_serde]
pub struct UnbondingDelegation {
	pub delegator_address: String,
    pub validator_address: String,
    pub entries: Vec<UnbondingDelegationEntry>,
}

#[cw_serde]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    pub unbonding_responses: Vec<UnbondingDelegation>,
}

#[cw_serde]
pub struct QueryDelegatorValidatorsResponse {
    pub validators: Vec<ValidatorDetail>,
}
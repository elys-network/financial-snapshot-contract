#[allow(unused_imports)]
use super::query_resp::*;

#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(QueryBalanceResponse)]
    BalanceOfDenom { address: String, denom: String },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_balance(address: String, denom: String) -> Self {
        ElysQuery::BalanceOfDenom{ address, denom }
    }
}
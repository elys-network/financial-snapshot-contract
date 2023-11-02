#[allow(unused_imports)]
use super::query_resp::*;

#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(QueryBalanceResponse)]
    QueryBalanceRequest { address: String, asset: String },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_balance(address: String, asset: String) -> Self {
        ElysQuery::QueryBalanceRequest{ address, asset }
    }
}
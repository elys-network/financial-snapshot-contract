use cosmwasm_std::{Coin, QuerierWrapper, QueryRequest, StdResult};

use super::{
    query::ElysQuery,
    query_resp::{QueryBalanceResponse,QueryDelegatorDelegationsResponse},
};

#[allow(dead_code)]
pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

#[allow(dead_code)]
impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }
    pub fn get_balance(&self, address: String, denom: String) -> StdResult<Coin> {
        let balance_query = ElysQuery::BalanceOfDenom {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(balance_query);
        let resp: QueryBalanceResponse = self.querier.query(&request)?;
        Ok(resp.balance)
    }

    pub fn get_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorDelegationsResponse> {
        let delegations_query = ElysQuery::Delegations {
            delegator_addr: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(delegations_query);
        let resp: QueryDelegatorDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}

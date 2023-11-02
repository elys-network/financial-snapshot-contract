use cosmwasm_std::{Coin, QuerierWrapper, QueryRequest, StdResult};

use super::{
    query::ElysQuery,
    query_resp::QueryBalanceResponse,
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
    pub fn get_balance(&self, address: String, asset: String) -> StdResult<Coin> {
        let balance_query = ElysQuery::QueryBalanceRequest {
            address: address.to_owned(),
            asset: asset.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(balance_query);
        let resp: QueryBalanceResponse = self.querier.query(&request)?;
        Ok(resp.balance)
    }
}

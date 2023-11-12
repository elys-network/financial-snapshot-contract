use cosmwasm_std::{Coin, QuerierWrapper, QueryRequest, StdResult};

use super::{
    query::ElysQuery,
    query_resp::{QueryBalanceResponse,
        QueryDelegatorDelegationsResponse, 
        QueryDelegatorUnbondingDelegationsResponse,
        QueryDelegatorValidatorsResponse,
        QueryShowCommitmentsResponse},
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
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(delegations_query);
        let resp: QueryDelegatorDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unbonding_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorUnbondingDelegationsResponse> {
        let unbonding_delegations_query = ElysQuery::UnbondingDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unbonding_delegations_query);
        let resp: QueryDelegatorUnbondingDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::AllValidators{ 
            delegator_address: delegator.to_owned()
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegator_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::DelegatorValidators{
            delegator_address: delegator.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_commitments(&self, address: String) -> StdResult<QueryShowCommitmentsResponse> {
        let commitments_query = ElysQuery::CommitmentShowCommitments{
            creator: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(commitments_query);
        let resp: QueryShowCommitmentsResponse = self.querier.query(&request)?;
        Ok(resp)
    }
}

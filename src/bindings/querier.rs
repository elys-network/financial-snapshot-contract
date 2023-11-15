use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use super::{
    query::ElysQuery,
    query_resp::{QueryDelegatorDelegationsResponse, 
        QueryDelegatorUnbondingDelegationsResponse,
        QueryDelegatorValidatorsResponse,
        QueryShowCommitmentsResponse, 
        QueryStakedPositionResponse,
        QueryUnstakedPositionResponse,
        QueryVestingInfoResponse},
};

use crate::types::{BalanceAvailable, BalanceBorrowed, EarnType};

#[allow(dead_code)]
pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

#[allow(dead_code)]
impl<'a> ElysQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, ElysQuery>) -> Self {
        ElysQuerier { querier }
    }

    pub fn get_balance(&self, address: String, denom: String) -> StdResult<BalanceAvailable> {
        let balance_query = ElysQuery::AmmBalance {
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorDelegationsResponse> {
        let delegations_query = ElysQuery::CommitmentDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(delegations_query);
        let resp: QueryDelegatorDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unbonding_delegations(&self, delegator_addr: String) -> StdResult<QueryDelegatorUnbondingDelegationsResponse> {
        let unbonding_delegations_query = ElysQuery::CommitmentUnbondingDelegations {
            delegator_address: delegator_addr.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unbonding_delegations_query);
        let resp: QueryDelegatorUnbondingDelegationsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_all_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentAllValidators{ 
            delegator_address: delegator.to_owned()
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(validators_query);
        let resp: QueryDelegatorValidatorsResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_delegator_validators(&self, delegator: String) -> StdResult<QueryDelegatorValidatorsResponse> {
        let validators_query = ElysQuery::CommitmentDelegatorValidators{
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

    pub fn get_staked_balance(&self, address: String, denom: String)-> StdResult<BalanceAvailable> {
        let staked_balance_query = ElysQuery::CommitmentStakedBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_rewards_balance(&self, address: String, denom: String) -> StdResult<BalanceAvailable> {
        let rewards_balance_query = ElysQuery::CommitmentRewardsBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(rewards_balance_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_borrowed_balance(&self, address: String) -> StdResult<BalanceBorrowed> {
        let borrowed_balance_query = ElysQuery::StableStakeBalanceOfBorrow{
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(borrowed_balance_query);
        let resp: BalanceBorrowed = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_staked_positions(&self, address: String) -> StdResult<QueryStakedPositionResponse> {
        let staked_position_query = ElysQuery::CommitmentStakedPositions{
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(staked_position_query);
        let resp: QueryStakedPositionResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_unstaked_positions(&self, address: String) -> StdResult<QueryUnstakedPositionResponse> {
        let unstaked_position_query = ElysQuery::CommitmentUnStakedPositions{
            delegator_address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(unstaked_position_query);
        let resp: QueryUnstakedPositionResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_vesting_info(&self, address: String) -> StdResult<QueryVestingInfoResponse> {
        let vesting_info_query = ElysQuery::CommitmentVestingInfo{
            address: address.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(vesting_info_query);
        let resp: QueryVestingInfoResponse = self.querier.query(&request)?;
        Ok(resp)
    }

    pub fn get_sub_bucket_rewards_balance(&self, address: String, denom: String, program: EarnType) -> StdResult<BalanceAvailable> {
        let sub_bucket_reward_query = ElysQuery::CommitmentRewardsSubBucketBalanceOfDenom{
            address: address.to_owned(),
            denom: denom.to_owned(),
            program: program.to_owned(),
        };
        let request: QueryRequest<ElysQuery> = QueryRequest::Custom(sub_bucket_reward_query);
        let resp: BalanceAvailable = self.querier.query(&request)?;
        Ok(resp)
    }

}

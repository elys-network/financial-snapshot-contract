#[allow(unused_imports)]
use super::query_resp::*;
use crate::types::{BalanceAvailable, BalanceBorrowed};

#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(QueryBalanceResponse)]
    AmmBalance { address: String, denom: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    CommitmentDelegations { delegator_address: String},
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    CommitmentUnbondingDelegations { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentAllValidators { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    CommitmentDelegatorValidators { delegator_address: String },
    #[returns(BalanceAvailable)]
    CommitmentStakedBalanceOfDenom { address: String, denom: String },
    #[returns(BalanceAvailable)]
    CommitmentRewardsBalanceOfDenom { address: String, denom: String },
    #[returns(QueryShowCommitmentsResponse)]
    CommitmentShowCommitments { creator: String },
    #[returns(BalanceBorrowed)]
    StableStakeBalanceOfBorrow { address: String },
}

impl CustomQuery for ElysQuery {}
impl ElysQuery {
    pub fn get_balance(address: String, denom: String) -> Self {
        ElysQuery::AmmBalance{ address, denom }
    }
    pub fn get_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegations{ delegator_address: delegator_addr }
    }
    pub fn get_unbonding_delegations(delegator_addr: String) -> Self {
        ElysQuery::CommitmentUnbondingDelegations{ delegator_address: delegator_addr }
    }
    pub fn get_all_validators() -> Self {
        ElysQuery::CommitmentAllValidators{ delegator_address: "".to_string() }
    }
    pub fn get_delegator_validators(delegator_addr: String) -> Self {
        ElysQuery::CommitmentDelegatorValidators{ delegator_address: delegator_addr }
    }
    pub fn get_commitments(address: String) -> Self {
        ElysQuery::CommitmentShowCommitments{ creator: address }
    }
    pub fn get_staked_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentStakedBalanceOfDenom{ address, denom }
    }
    pub fn get_rewards_balance(address: String, denom: String) -> Self {
        ElysQuery::CommitmentRewardsBalanceOfDenom{ address, denom }
    }
    pub fn get_borrowed_balance(address: String) -> Self {
        ElysQuery::StableStakeBalanceOfBorrow{ address }
    }
}
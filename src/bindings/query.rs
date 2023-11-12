#[allow(unused_imports)]
use super::query_resp::*;

#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum ElysQuery {
    #[returns(QueryBalanceResponse)]
    AmmBalance { address: String, denom: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    Delegations { delegator_address: String},
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    UnbondingDelegations { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    AllValidators { delegator_address: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    DelegatorValidators { delegator_address: String },
    #[returns(QueryBalanceResponse)]
    StakedBalanceOfDenom { address: String, denom: String },
    #[returns(QueryBalanceResponse)]
    RewardsBalanceOfDenom { address: String, denom: String },
    #[returns(QueryShowCommitmentsResponse)]
    CommitmentShowCommitments { creator: String },
}

impl CustomQuery for ElysQuery {}

impl ElysQuery {
    pub fn get_balance(address: String, denom: String) -> Self {
        ElysQuery::AmmBalance{ address, denom }
    }
    pub fn get_delegations(delegator_addr: String) -> Self {
        ElysQuery::Delegations{ delegator_address: delegator_addr }
    }
    pub fn get_unbonding_delegations(delegator_addr: String) -> Self {
        ElysQuery::UnbondingDelegations{ delegator_address: delegator_addr }
    }
    pub fn get_all_validators() -> Self {
        ElysQuery::AllValidators{ delegator_address: "".to_string() }
    }
    pub fn get_delegator_validators(delegator_addr: String) -> Self {
        ElysQuery::DelegatorValidators{ delegator_address: delegator_addr }
    }
    pub fn get_commitments(address: String) -> Self {
        ElysQuery::CommitmentShowCommitments{ creator: address }
    }
}
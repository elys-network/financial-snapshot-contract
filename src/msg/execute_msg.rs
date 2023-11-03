use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Int128};

#[cw_serde]
pub enum ExecuteMsg {
    StakeRequest {
        address: String,
        amount: u64,
        asset: String,
        validator_address: Option<String>,
    },
    UnstakeRequest {
        address: String,
        amount: u64,
        asset: String,
        validator_address: Option<String>,
    },
    ElysRedelegateRequest {
        delegator_address:    String,
        validator_src_address: String,
        validator_dst_address: String,
        amount:              Coin,
    },
    ElysCancelUnstakeRequest {
        delegator_address: String,
        validator_address: String,
        // amount is always less than or equal to unbonding delegation entry balance
        amount: Coin,
        // creation_height is the height which the unbonding took place.
        creation_height: i64,
    },
    EdenVestRequest {
        creator: String,
        amount:  u64,
    },
    EdenCancelVestRequest {
        creator: String,
        amount:  u64,
    },
    ClaimRewardsRequest {
        delegator_address: String,
        denom:            String,
    },
    ClaimValidatorCommissionRequest {
        delegator_address: String,
        validator_address: String,
        denom:            String,
    }
}

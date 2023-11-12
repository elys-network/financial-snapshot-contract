use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, Int128, Coin};

#[cw_serde]
pub enum ElysMsg {
    CommitmentStake {
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    CommitmentUnstake {
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    IncentiveBeginRedelegate {
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    },
    IncentiveCancelUnbondingDelegation {
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    },
    CommitmentVest {
        creator: String,
        amount:  Int128,
        denom:   String,
    },
    CommitmentCancelVest {
        creator: String,
        amount:  Int128,
        denom:   String,
    },
    IncentiveWithdrawRewards {
        delegator_address: String,
        denom:            String,
    },
    IncentiveWithdrawValidatorCommission {
        delegator_address: String,
        validator_address: String,
        denom:            String,
    }
}

impl ElysMsg {
    pub fn stake_token(
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentStake {
            creator:address.to_owned(),
            amount:amount.to_owned(),
            asset:asset.to_owned(),
            validator_address:validator_address.to_owned(),
        }
    }

    pub fn unstake_token(
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentUnstake {
            creator:address.to_owned(),
            amount:amount.to_owned(),
            asset:asset.to_owned(),
            validator_address:validator_address.to_owned(),
        }
    }

    pub fn begin_redelegate(
        delegator_address:    String,
        validator_src_address: String,
        validator_dst_address: String,
        amount:              Coin,
    ) -> Self {
        Self::IncentiveBeginRedelegate {
            delegator_address: delegator_address.to_owned(),
            validator_src_address: validator_src_address.to_owned(),
            validator_dst_address: validator_dst_address.to_owned(),
            amount: amount.to_owned(),
        }
    }

    pub fn cancel_unbonding(
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    ) -> Self {
        Self::IncentiveCancelUnbondingDelegation {
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
            amount: amount.to_owned(),
            creation_height: creation_height.to_owned(),
        }
    }
    
    pub fn eden_vesting(
        creator: String,
        amount:  Int128,
        denom:   String,
    ) -> Self {
        Self::CommitmentVest {
            creator: creator.to_owned(),
            amount:  amount,
            denom:   denom.to_owned(),
        }
    }    
    
    pub fn eden_cancel_vesting(
        creator: String,
        amount:  Int128,
        denom:   String,
    ) -> Self {
        Self::CommitmentCancelVest {
            creator: creator.to_owned(),
            amount:  amount,
            denom:   denom.to_owned(),
        }
    }
        
    pub fn withdraw_rewards(
        delegator_address: String,
        denom:            String,
    ) -> Self {
        Self::IncentiveWithdrawRewards {
            delegator_address: delegator_address.to_owned(),
            denom: denom.to_owned(),
        }
    }

    pub fn withdraw_validator_commissions(
        delegator_address: String,
        validator_address: String,
        denom:            String,
    ) -> Self {
        Self::IncentiveWithdrawValidatorCommission {
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
            denom: denom.to_owned(),
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}

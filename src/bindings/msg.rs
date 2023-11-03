use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, Int128, Coin};

#[cw_serde]
pub enum ElysMsg {
    MsgStake {
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    MsgUnstake {
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    MsgBeginRedelegate {
        delegator_address:    String,
        validator_src_address: String,
        validator_dst_address: String,
        amount:              Coin,
    },
    MsgCancelUnbondingDelegation {
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: u64,
    },
    MsgVest {
        creator: String,
        amount:  Int128,
        denom:   String,
    },
    MsgCancelVest {
        creator: String,
        amount:  Int128,
        denom:   String,
    },
    MsgWithdrawRewards {
        delegator_address: String,
        denom:            String,
    },
    MsgWithdrawValidatorCommission {
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
        Self::MsgStake {
            address:address.to_owned(),
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
        Self::MsgUnstake {
            address:address.to_owned(),
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
        Self::MsgBeginRedelegate {
            delegator_address: delegator_address.to_owned(),
            validator_src_address: validator_src_address.to_owned(),
            validator_dst_address: validator_dst_address.to_owned(),
            amount: amount.clone().to_owned(),
        }
    }

    pub fn cancel_unbonding(
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: u64,
    ) -> Self {
        Self::MsgCancelUnbondingDelegation {
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
            amount: amount.clone().to_owned(),
            creation_height: creation_height,
        }
    }
    
    pub fn eden_vesting(
        creator: String,
        amount:  Int128,
        denom:   String,
    ) -> Self {
        Self::MsgVest {
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
        Self::MsgCancelVest {
            creator: creator.to_owned(),
            amount:  amount,
            denom:   denom.to_owned(),
        }
    }
        
    pub fn withdraw_rewards(
        delegator_address: String,
        denom:            String,
    ) -> Self {
        Self::MsgWithdrawRewards {
            delegator_address: delegator_address.to_owned(),
            denom:            denom.to_owned(),
        }
    }

    pub fn withdraw_validator_commissions(
        delegator_address: String,
        validator_address: String,
        denom:            String,
    ) -> Self {
        Self::MsgWithdrawValidatorCommission {
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

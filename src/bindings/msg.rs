use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, Int128};

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
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}

use crate::types::EarnDetail;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetEarnDetailsResp {
    pub data: EarnDetail,
}

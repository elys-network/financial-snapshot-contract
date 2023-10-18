use crate::types::VestingDetail;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetVestingDetailsResp {
    pub vesting_details: Vec<VestingDetail>,
}

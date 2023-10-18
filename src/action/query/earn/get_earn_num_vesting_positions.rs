use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetNumVestingPositionsResp};

// TODO
// Query Elys commitment module to get number of vestings per user
// For now returning 0,
pub fn get_earn_num_vesting_positions(_deps: Deps<ElysQuery>, _address: String) -> Result<GetNumVestingPositionsResp, ContractError> {
    let resp = GetNumVestingPositionsResp {
        num_vesting_positions: 0,
    };

    Ok(resp)
}

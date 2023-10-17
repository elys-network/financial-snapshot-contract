use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetVestingDetailsResp};

pub fn get_earn_vesting_details(deps: Deps<ElysQuery>, address: String) -> Result<GetVestingDetailsResp, ContractError> {
    let ret = VESTING_DETAILS.may_load(deps.storage, &address);
    let resp = GetVestingDetailsResp {
        vesting_details: match ret {
            Ok(Some(data)) => data.to_owned(),
            Ok(None) => VestingDetail::new_dummys(),
            Err(_) => return Err(ContractError::RewardError{}),
        },
    };

    Ok(resp)
}

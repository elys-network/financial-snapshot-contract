use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetEdenBoostEarnProgramResp};
use crate::types::earn_program::EdenBoostEarnProgram;

pub fn get_eden_boost_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    if asset != "uedenb" {
        return Err(ContractError::AssetDenomError{});
    }

    // TODO:
    // Backend logic implementation
    let resp = GetEdenBoostEarnProgramResp {
        data: EdenBoostEarnProgram::edenb_dummy(address, asset),
    };

    Ok(resp)
}
use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetEdenEarnProgramResp};
use crate::types::earn_program::EdenEarnProgram;

pub fn get_eden_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenEarnProgramResp, ContractError> {
    if asset != "ueden" {
        return Err(ContractError::AssetDenomError{});
    }

    // TODO:
    // Real backend logic implementation
    let resp = GetEdenEarnProgramResp {
        data: EdenEarnProgram::eden_dummy(address, asset),
    };

    Ok(resp)
}
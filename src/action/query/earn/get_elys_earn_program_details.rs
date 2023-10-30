use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetElysEarnProgramResp};
use crate::types::earn_program::ElysEarnProgram;

pub fn get_elys_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetElysEarnProgramResp, ContractError> {
    if asset != "uelys" {
        return Err(ContractError::AssetDenomError{});
    }

    // TODO:
    // Real backend logic implementation
    let resp = GetElysEarnProgramResp {
        data: ElysEarnProgram::elys_dummy(address, asset)
    };

    Ok(resp)
}
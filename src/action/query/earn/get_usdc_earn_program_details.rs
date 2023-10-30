use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetUSDCEarnProgramResp};
use crate::types::earn_program::USDCEarnProgram;

pub fn get_usdc_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetUSDCEarnProgramResp, ContractError> {
    // TODO:
    // Real backend logic implementation
    let resp = GetUSDCEarnProgramResp {
        data: USDCEarnProgram::usdc_dummy(address, asset)

    };

    Ok(resp)
}
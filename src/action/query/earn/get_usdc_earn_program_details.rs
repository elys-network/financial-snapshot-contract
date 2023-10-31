use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetUsdcEarnProgramResp};
use crate::types::earn_program::UsdcEarnProgram;

pub fn get_usdc_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetUsdcEarnProgramResp, ContractError> {
    if asset != "uusdc" {
        return Err(ContractError::AssetDenomError{});
    }

    // TODO:
    // Real backend logic implementation
    let resp = GetUsdcEarnProgramResp {
        data: UsdcEarnProgram::usdc_dummy(address, asset)

    };

    Ok(resp)
}
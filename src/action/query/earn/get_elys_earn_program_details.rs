use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetElysEarnProgramResp};
use crate::types::earn_program::ElysEarnProgram;

pub fn get_elys_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetElysEarnProgramResp, ContractError> {
    if asset != "uelys" {
        return Err(ContractError::AssetDenomError{});
    }

    // TODO:
    // 1. address valid and asset valid -> return earn detail.
    // 2. address valid and asset invalid -> return earn detail of all asset.
    // 3. address not provider -> return all earn details.
    let resp = GetElysEarnProgramResp {
        data: ElysEarnProgram::elys_dummy(address, asset)
    };

    Ok(resp)
}
use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetEarnDetailsResp, types::EarnDetail};

pub fn get_usdc_earn_program_details(_deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEarnDetailsResp, ContractError> {
    // TODO:
    // 1. address valid and asset valid -> return earn detail.
    // 2. address valid and asset invalid -> return earn detail of all asset.
    // 3. address not provider -> return all earn details.
    let resp = GetEarnDetailsResp {
        data: match address {
            Some(_addr) => EarnDetail::new_dummy_uusdc(asset, address),
            None => EarnDetail::new_dummy_uusdc(asset, address),
        }
    };

    Ok(resp)
}
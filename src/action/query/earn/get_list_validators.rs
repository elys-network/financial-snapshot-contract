use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetListOfValidatorsResp, types::ListOfValidators};

pub fn get_list_validators(_deps: Deps<ElysQuery>, name: Option<String>) -> Result<GetListOfValidatorsResp, ContractError> {
    // TODO:
    // 1. address valid and asset valid -> return earn detail.
    // 2. address valid and asset invalid -> return earn detail of all asset.
    // 3. address not provider -> return all earn details.
    let resp = GetListOfValidatorsResp {
        data: match address {
            Some(_addr) => EarnDetail::new_dummy_uusdc(asset, address),
            None => EarnDetail::new_dummy_uusdc(asset, address),
        }
    };

    Ok(resp)
}
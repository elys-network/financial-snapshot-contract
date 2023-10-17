use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::common::GetAPRResp};

pub fn get_apr(deps: Deps<ElysQuery>, asset: String) -> Result<GetAPRResp, ContractError> {
    let resp = GetAPRResp {
        apr: 100,
    };

    Ok(resp)
}

use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::common::GetAPRResp};

pub fn get_apr(_deps: Deps<ElysQuery>, _asset: String) -> Result<GetAPRResp, ContractError> {
    let resp = GetAPRResp {
        apr: 100,
    };

    Ok(resp)
}

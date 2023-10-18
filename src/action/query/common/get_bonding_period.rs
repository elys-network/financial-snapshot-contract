use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::common::GetBondingPeriodResp};

pub fn get_bonding_period(_deps: Deps<ElysQuery>, _asset: String) -> Result<GetBondingPeriodResp, ContractError> {
    let resp = GetBondingPeriodResp {
        bonding_period: 0,
    };

    Ok(resp)
}

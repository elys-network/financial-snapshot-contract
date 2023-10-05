use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetTotalBalanceResp, types::*};

pub fn get_total_balance(deps: Deps<ElysQuery>, address: String) -> Result<GetTotalBalanceResp, ContractError> {
    let total_balance: TotalBalance = TOTAL_BALANCE.load(deps.storage, &address)?;
    let resp = GetTotalBalanceResp {
        data: total_balance,
    };

    Ok(resp)
}

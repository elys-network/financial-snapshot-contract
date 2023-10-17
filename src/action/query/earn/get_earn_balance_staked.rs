use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetBalanceResp};

pub fn get_earn_balance_staked(deps: Deps<ElysQuery>, address: String, asset: String) -> Result<GetBalanceResp, ContractError> {
    let ret = BALANCE_STAKED.may_load(deps.storage, &address);
    let resp: GetBalanceResp;
   
    match ret {
        Ok(Some(data)) => {
            let have_assets = data.iter().find(|balance| balance.asset == asset);
            resp = GetBalanceResp {
                balance: match have_assets {
                    Some(b) => b.to_owned(),
                    None => BalanceDollar::new_dummy(),
                },
            };
        },
        Ok(None) => {
            resp = GetBalanceResp {
                balance: BalanceDollar::new_dummy(),
            };
        },
        Err(_) => {
            return Err(ContractError::RewardError{});
        }
    }

    Ok(resp)
}

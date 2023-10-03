use crate::bindings::query::ElysQuery;

use super::*;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetPortfolio { address } => Ok(to_binary(&query::get_portfolio(deps, address)?)?),
        GetTotalBalance { address } => Ok(to_binary(&query::get_total_balance(deps, address)?)?),
    }
}

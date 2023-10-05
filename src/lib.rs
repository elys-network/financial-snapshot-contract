pub mod entry_point {
    use crate::action;
    use crate::error::ContractError;
    use crate::msg;
    use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    mod instantiate;
    mod query;

    pub use instantiate::instantiate;
    pub use query::query;
}

mod bindings {
    pub mod querier;
    pub mod query;
}

pub mod msg {
    mod instantiate_msg;
    mod query_msg;

    pub use instantiate_msg::InstantiateMsg;
    pub use query_msg::QueryMsg;
    pub mod query_resp {
        mod get_portfolio_resp;
        pub use get_portfolio_resp::GetPortfolioResp;
        mod get_total_balance_resp;
        pub use get_total_balance_resp::GetTotalBalanceResp;
    }
}

pub mod types {
    mod portfolio {
        pub mod portfolio;
        mod impls {
            mod init;
            #[cfg(test)]
            mod new_dummy;
        }
    }
    pub use portfolio::portfolio::Portfolio;

    mod total_balance {
        pub mod total_balance;
        mod impls {
            mod init;
            #[cfg(test)]
            mod new_dummy;
        }
    }
    pub use total_balance::total_balance::TotalBalance;

    pub mod page_request;
    pub use page_request::PageRequest;
    pub mod page_response;
    pub use page_response::PageResponse;
}

mod error;
use bindings::query::ElysQuery;
pub use error::ContractError;

mod states {
    mod portfolio;
    pub use portfolio::PORTFOLIO;
    
    mod total_balance;
    pub use total_balance::TOTAL_BALANCE;
}

mod action {
    use crate::{states::PORTFOLIO, states::TOTAL_BALANCE, ContractError};

    pub mod query {
        use super::*;
        use cosmwasm_std::Deps;
        mod get_portfolio;
        pub use get_portfolio::get_portfolio;
        mod get_total_balance;
        pub use get_total_balance::get_total_balance;
    }
}

#[cfg(test)]
mod tests;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    entry_point::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    entry_point::query(deps, env, msg)
}

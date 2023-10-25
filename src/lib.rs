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
        pub mod pod {
            mod get_portfolio_resp;
            mod get_total_balance_resp;
            mod get_liquid_asset_resp;
            mod get_liquid_assets_resp;
            mod get_rewards_resp;
            mod get_liquidity_position_resp;
            mod get_liquidity_positions_resp;

            pub use get_portfolio_resp::GetPortfolioResp;
            pub use get_total_balance_resp::GetTotalBalanceResp;
            pub use get_liquid_asset_resp::GetLiquidAssetResp;
            pub use get_liquid_assets_resp::GetLiquidAssetsResp;
            pub use get_rewards_resp::GetRewardsResp;
            pub use get_liquidity_position_resp::GetLiquidityPositionResp;
            pub use get_liquidity_positions_resp::GetLiquidityPositionsResp;
        }
        pub mod earn {
            mod get_earn_details_resp;
            pub use get_earn_details_resp::GetEarnDetailsResp;
        }
    }
}

pub mod types {
    mod portfolio {
        pub mod portfolio;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use portfolio::portfolio::Portfolio;

    mod total_balance {
        pub mod total_balance;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use total_balance::total_balance::TotalBalance;
    
    mod liquid_asset {
        pub mod liquid_asset;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use liquid_asset::liquid_asset::LiquidAsset;

    mod reward {
        pub mod reward;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use reward::reward::Reward;

    mod liquidity_position {
        pub mod liquidity_position;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use liquidity_position::liquidity_position::LiquidityPosition;

    mod earn_detail {
        pub mod earn_detail;
        mod impls {
            mod init;
            mod new_dummy;
        }
    }
    pub use earn_detail::earn_detail::EarnDetail;

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

    mod rewards;
    pub use rewards::REWARDS;
    
    mod liquid_assets;
    pub use liquid_assets::LIQUID_ASSETS;

    mod liquidity_positions;
    pub use liquidity_positions::LIQUIDITY_POSITIONS;

    mod earn_details;
    pub use earn_details::EARN_DETAILS;
}

mod action {
    pub mod query {
        pub mod pod {
            mod get_pod_liquid_asset;
            mod get_pod_liquid_assets;
            mod get_pod_liquidity_position;
            mod get_pod_liquidity_positions;
            mod get_pod_portfolio;
            mod get_pod_rewards;
            mod get_pod_total_balance;
    
            use cosmwasm_std::Deps;
            use crate::{states::PORTFOLIO, states::TOTAL_BALANCE, states::REWARDS, states::LIQUID_ASSETS, states::LIQUIDITY_POSITIONS, types::*, ContractError};
            pub use get_pod_liquid_asset::get_pod_liquid_asset;
            pub use get_pod_liquid_assets::get_pod_liquid_assets;
            pub use get_pod_liquidity_position::get_pod_liquidity_position;
            pub use get_pod_liquidity_positions::get_pod_liquidity_positions;
            pub use get_pod_portfolio::get_pod_portfolio;
            pub use get_pod_rewards::get_pod_rewards;
            pub use get_pod_total_balance::get_pod_total_balance;
        }

        pub mod earn {
            mod get_earn_details;
    
            use cosmwasm_std::Deps;
            use crate::ContractError;
            pub use get_earn_details::get_earn_details;
        }
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

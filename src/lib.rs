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
        mod get_liquid_asset_resp;
        pub use get_liquid_asset_resp::GetLiquidAssetResp;
        mod get_liquid_assets_resp;
        pub use get_liquid_assets_resp::GetLiquidAssetsResp;
        mod get_rewards_resp;
        pub use get_rewards_resp::GetRewardsResp;
        mod get_liquidity_position_resp;
        pub use get_liquidity_position_resp::GetLiquidityPositionResp;
        mod get_liquidity_positions_resp;
        pub use get_liquidity_positions_resp::GetLiquidityPositionsResp;
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
}

mod action {
    use crate::{states::PORTFOLIO, states::TOTAL_BALANCE, states::REWARDS, states::LIQUID_ASSETS, states::LIQUIDITY_POSITIONS, types::*, ContractError};

    pub mod query {
        mod get_portfolio;
        mod get_total_balance;
        mod get_rewards;
        mod get_liquid_asset;
        mod get_liquid_assets;
        mod get_liquidity_position;
        mod get_liquidity_positions;

        use super::*;
        use cosmwasm_std::Deps;
        pub use get_portfolio::get_portfolio;
        pub use get_total_balance::get_total_balance;
        pub use get_rewards::get_rewards;
        pub use get_liquid_asset::get_liquid_asset;
        pub use get_liquid_assets::get_liquid_assets;
        pub use get_liquidity_position::get_liquidity_position;
        pub use get_liquidity_positions::get_liquidity_positions;
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

use crate::bindings::query::ElysQuery;

use super::*;
use msg::QueryMsg;

pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query::pod;
    use action::query::earn;
    use QueryMsg::*;

    match msg {
        // Pod dashboard
        GetPodPortfolio { address } => Ok(to_binary(&pod::get_pod_portfolio(deps, address)?)?),
        GetPodTotalBalance { address } => Ok(to_binary(&pod::get_pod_total_balance(deps, address)?)?),
        GetPodRewards { address } => Ok(to_binary(&pod::get_pod_rewards(deps, address)?)?),
        GetPodLiquidAssets { } => Ok(to_binary(&pod::get_pod_liquid_assets(deps)?)?),
        GetPodLiquidAsset { asset } => Ok(to_binary(&pod::get_pod_liquid_asset(deps, asset)?)?),
        GetPodLiquidityPositions { } => Ok(to_binary(&pod::get_pod_liquidity_positions(deps)?)?),
        GetPodLiquidityPosition { pool_id } => Ok(to_binary(&pod::get_pod_liquidity_position(deps, pool_id)?)?),

        // Earn dashboard
        GetEarnDetails { address, asset } => Ok(to_binary(&earn::get_earn_details(deps, address, asset)?)?),
    }
}

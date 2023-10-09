use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetLiquidAssetsResp, types::*};

pub fn get_liquid_assets(deps: Deps<ElysQuery>) -> Result<GetLiquidAssetsResp, ContractError> {
    let liquid_assets: Vec<LiquidAsset> = LIQUID_ASSETS.load(deps.storage)?;
    let resp = GetLiquidAssetsResp {
        liquid_assets: liquid_assets,
    };

    Ok(resp)
}

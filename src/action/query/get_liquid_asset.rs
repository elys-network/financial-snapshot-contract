use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetLiquidAssetsResp, types::*};

pub fn get_liquid_asset(deps: Deps<ElysQuery>, asset: String) -> Result<GetLiquidAssetsResp, ContractError> {
    let liquid_assets: Vec<LiquidAsset> = LIQUID_ASSETS.load(deps.storage)?;
    let have_assets = liquid_assets.iter().find(|liquid_asset| liquid_asset.asset == asset);

    let resp = GetLiquidAssetsResp {
        liquid_assets: have_assets,
    };

    Ok(resp)
}

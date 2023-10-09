use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetLiquidAssetResp, types::*, states::LIQUID_ASSETS};

pub fn get_liquid_asset(deps: Deps<ElysQuery>, asset: String) -> Result<GetLiquidAssetResp, ContractError> {
    let liquid_assets: Vec<LiquidAsset> = LIQUID_ASSETS.load(deps.storage)?;
    let have_assets = liquid_assets.iter().find(|liquid_asset| liquid_asset.asset == asset);
    let resp = GetLiquidAssetResp {
        liquid_asset: match have_assets {
            Some(liquid_asset) => liquid_asset.to_owned(),
            None => LiquidAsset::new_dummy(),
        },
    };

    Ok(resp)
}

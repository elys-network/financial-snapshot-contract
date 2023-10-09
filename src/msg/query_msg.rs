#[allow(unused_imports)]
use super::query_resp::*;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetTotalBalanceResp)]
    GetTotalBalance { address: String },
    #[returns(GetPortfolioResp)]
    GetPortfolio { address: String },
    #[returns(GetRewardsResp)]
    GetRewards { address: String },
    #[returns(GetLiquidAssetsResp)]
    GetLiquidAssets { },
    #[returns(GetLiquidAssetResp)]
    GetLiquidAsset { asset: String },
}

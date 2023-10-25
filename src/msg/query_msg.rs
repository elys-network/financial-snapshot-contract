#[allow(unused_imports)]
use super::query_resp::pod::*;
#[allow(unused_imports)]
use super::query_resp::earn::*;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Pod dashboard
    #[returns(GetTotalBalanceResp)]
    GetPodTotalBalance { address: String },
    #[returns(GetPortfolioResp)]
    GetPodPortfolio { address: String },
    #[returns(GetRewardsResp)]
    GetPodRewards { address: String },
    #[returns(GetLiquidAssetsResp)]
    GetPodLiquidAssets { },
    #[returns(GetLiquidAssetResp)]
    GetPodLiquidAsset { asset: String },
    #[returns(GetLiquidityPositionsResp)]
    GetPodLiquidityPositions { },
    #[returns(GetLiquidityPositionResp)]
    GetPodLiquidityPosition { pool_id: u64 },

    // Earn dashboard
    #[returns(GetEarnDetailsResp)]
    GetEarnDetails { address: Option<String>, asset: Option<String> },
}

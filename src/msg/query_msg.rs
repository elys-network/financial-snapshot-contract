#[allow(unused_imports)]
use super::query_resp::pod::*;
use super::query_resp::earn::*;
use super::query_resp::common::*;
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
    #[returns(GetAPRResp)]
    GetApr { asset: String },
    #[returns(GetBondingPeriodResp)]
    GetBondingPeriod { asset: String },
    #[returns(GetBalanceResp)]
    GetEarnBalanceAvailable { address: String, asset: String },
    #[returns(GetBalanceResp)]
    GetEarnBalanceStaked { address: String, asset: String },
    #[returns(GetBalanceResp)]
    GetEarnBalanceVesting { address: String, asset: String },
    #[returns(GetBalanceResp)]
    GetEarnBalanceReward { address: String, asset: String },
    #[returns(GetNumVestingPositionsResp)]
    GetEarnNumVestingPositions { address: String },
    #[returns(GetVestingDetailsResp)]
    GetEarnVestingDetails { address: String },
}

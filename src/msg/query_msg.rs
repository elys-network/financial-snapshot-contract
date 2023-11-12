#[allow(unused_imports)]
use super::query_resp::pod::*;
#[allow(unused_imports)]
use super::query_resp::earn::*;
use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::bindings::query_resp::{QueryDelegatorDelegationsResponse, QueryDelegatorUnbondingDelegationsResponse, QueryDelegatorValidatorsResponse};

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
    #[returns(GetEdenEarnProgramResp)]
    GetEdenEarnProgramDetails { address: Option<String>, asset: String },
    #[returns(GetEdenBoostEarnProgramResp)]
    GetEdenBoostEarnProgramDetails { address: Option<String>, asset: String },
    #[returns(GetElysEarnProgramResp)]
    GetElysEarnProgramDetails { address: Option<String>, asset: String },
    #[returns(GetUsdcEarnProgramResp)]
    GetUsdcEarnProgramDetails { address: Option<String>, asset: String },
    #[returns(QueryDelegatorValidatorsResponse)]
    GetAllValidators { delegator_addr: Option<String> },
    #[returns(QueryDelegatorValidatorsResponse)]
    GetDelegatorValidators { delegator_addr: String },
    #[returns(QueryDelegatorDelegationsResponse)]
    GetDelegations { delegator_addr: String },
    #[returns(QueryDelegatorUnbondingDelegationsResponse)]
    GetUnbondingDelegations { delegator_addr: String },
}

use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetElysEarnProgramResp};
use crate::types::{earn_program::elys_earn::ElysEarnProgram, ElysDenom, BalanceReward, AprElys, EarnType};

pub fn get_elys_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetElysEarnProgramResp, ContractError> {
    let denom = ElysDenom::Elys.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let usdc_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Eden.as_str().to_string())?;
    let edenb_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::EdenBoost.as_str().to_string())?;

    let resp = GetElysEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Usdc.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let edenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let staked_positions = querier.get_staked_positions(addr.clone())?;
                let unstaked_positions = querier.get_unstaked_positions(addr.clone())?;

                ElysEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr,
                        uedenb: edenb_apr.apr,
                    },
                    available: Some(available),
                    staked: Some(staked),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: ElysDenom::Usdc.as_str().to_string(),
                            amount: usdc_rewards.amount,
                            usd_amount: Some(usdc_rewards.usd_amount),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: eden_rewards.amount,
                            usd_amount: Some(eden_rewards.usd_amount),
                        },
                        BalanceReward {
                            asset: ElysDenom::EdenBoost.as_str().to_string(),
                            amount: edenb_rewards.amount,
                            usd_amount: None,
                        },
                    ]),
                    staked_positions: staked_positions.staked_position,
                    unstaked_positions: unstaked_positions.unstaked_position,
                }
            },
            None => {
                ElysEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr,
                        uedenb: edenb_apr.apr,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                    staked_positions: None,
                    unstaked_positions: None,
                }
            }
        }
    };

    Ok(resp)
}
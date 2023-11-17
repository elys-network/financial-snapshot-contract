use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetEdenEarnProgramResp};
use crate::types::{earn_program::eden_earn::EdenEarnProgram, ElysDenom, BalanceReward, AprElys, EarnType};

pub fn get_eden_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenEarnProgramResp, ContractError> {
    let denom = ElysDenom::Eden.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
    let resp = GetEdenEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Usdc.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let edenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                let vesting_info = querier.get_vesting_info(addr.clone())?;

                EdenEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: 70,
                        ueden: 80,
                        uedenb: 100,
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
                    vesting: Some(vesting_info.vesting),
                    vesting_details: vesting_info.vesting_details,
                }
            },
            None => {
                EdenEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: 70,
                        ueden: 80,
                        uedenb: 100,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                    vesting: None,
                    vesting_details: None,
                }
            }
        }
    };

    Ok(resp)
}
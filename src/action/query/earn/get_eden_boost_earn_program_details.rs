use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetEdenBoostEarnProgramResp};
use crate::types::{earn_program::eden_boost_earn::EdenBoostEarnProgram, ElysDenom, BalanceReward, AprUsdc, EarnType};

pub fn get_eden_boost_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    let denom = ElysDenom::EdenBoost.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
    let resp = GetEdenBoostEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Usdc.as_str().to_string(), EarnType::EDENB_PROGRAM)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EDENB_PROGRAM)?;
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;

                EdenBoostEarnProgram {
                    bonding_period: 0,
                    apr: AprUsdc {
                        uusdc: 70,
                        ueden: 80,
                    },
                    available: Some(available.amount),
                    staked: Some(staked.amount),
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
                    ]),
                }
            },
            None => {
                EdenBoostEarnProgram {
                    bonding_period: 90,
                    apr: AprUsdc {
                        uusdc: 70,
                        ueden: 80,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                }
            }
        }
    };

    Ok(resp)
}
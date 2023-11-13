use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetUsdcEarnProgramResp};
use crate::types::{earn_program::usdc_earn::UsdcEarnProgram, ElysDenom};
use crate::types::{BalanceReward, AprUsdc};

pub fn get_usdc_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetUsdcEarnProgramResp, ContractError> {
    let denom = ElysDenom::Usdc.as_str();

    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }
    
    let querier = ElysQuerier::new(&deps.querier);
    let resp = GetUsdcEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_rewards_balance(addr.clone(), ElysDenom::Usdc.as_str().to_string())?;
                let eden_rewards = querier.get_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string())?;
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                let borrowed = querier.get_borrowed_balance(addr.clone())?;

                UsdcEarnProgram {
                    bonding_period: 90,
                    apr: AprUsdc {
                        uusdc: 70,
                        ueden: 80,
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
                    ]),
                    borrowed: Some(borrowed),
                }
            },
            None => UsdcEarnProgram {
                bonding_period: 90,
                apr: AprUsdc {
                    uusdc: 70,
                    ueden: 80,
                },
                available: None,
                staked: None,
                rewards: None,
                borrowed: None,
            }
        }
    };

    Ok(resp)
}
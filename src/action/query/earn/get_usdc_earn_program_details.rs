use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetUsdcEarnProgramResp};
use crate::types::{earn_program::usdc_earn::UsdcEarnProgram, ElysDenom};
use crate::types::{BalanceReward, AprUsdc, EarnType};
use cosmwasm_std::{coin, Decimal, Uint128};

pub fn get_usdc_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetUsdcEarnProgramResp, ContractError> {
    let denom = ElysDenom::Usdc.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }
    
    let querier = ElysQuerier::new(&deps.querier);
    let usdc_apr = querier.get_incentive_apr(EarnType::UsdcProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::UsdcProgram as i32, ElysDenom::Eden.as_str().to_string())?;

    let resp = GetUsdcEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Usdc.as_str().to_string(), EarnType::UsdcProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::UsdcProgram as i32)?;

                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                available.usd_amount = available.usd_amount.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();

                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                staked.usd_amount = staked.usd_amount.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                
                let mut borrowed = querier.get_borrowed_balance(addr.clone())?;
                borrowed.usd_amount = borrowed.usd_amount.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();

                // have value in usd
                let mut eden_rewards_in_usd = querier.get_amm_price_by_denom(coin(eden_rewards.amount.u128().clone(), ElysDenom::Elys.as_str().to_string()))?;
                eden_rewards_in_usd = eden_rewards_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                
                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();

                UsdcEarnProgram {
                    bonding_period: 1,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(available),
                    staked: Some(staked),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: ElysDenom::Usdc.as_str().to_string(),
                            amount: usdc_rewards.amount,
                            usd_amount: Some(usdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: eden_rewards.amount,
                            usd_amount: Some(eden_rewards_in_usd),
                        },
                    ]),
                    borrowed: Some(borrowed),
                }
            },
            None => UsdcEarnProgram {
                bonding_period: 90,
                apr: AprUsdc {
                    uusdc: usdc_apr.apr.to_owned(),
                    ueden: eden_apr.apr.to_owned(),
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
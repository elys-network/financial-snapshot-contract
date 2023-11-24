use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetElysEarnProgramResp};
use crate::types::{earn_program::elys_earn::ElysEarnProgram, ElysDenom, BalanceReward, AprElys, EarnType};
use cosmwasm_std::{coin, Decimal, Uint128};

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
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let staked_positions = querier.get_staked_positions(addr.clone())?;
                let unstaked_positions = querier.get_unstaked_positions(addr.clone())?;

                let usdc_usd_price = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();

                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_div(usdc_usd_price).unwrap();
                let elys_price_in_usd = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()))?;

                // have value in usd
                let mut eden_rewards_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_div(usdc_usd_price).unwrap();

                // have value in usd
                let mut available_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(available.amount, 0).unwrap()).unwrap();
                available_in_usd = available_in_usd.checked_div(usdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                let mut staked_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap()).unwrap();
                staked_in_usd = staked_in_usd.checked_div(usdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;
              
                ElysEarnProgram {
                    bonding_period: 14,
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
                            usd_amount: Some(usdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: eden_rewards.amount,
                            usd_amount: Some(eden_rewards_in_usd),
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
use crate::types::earn_program::usdc_earn::USDCEarnProgram;
use crate::types::{BalanceReward, AprUsdc, BalanceAvailable, BalanceBorrowed};

use cosmwasm_std::{Decimal, Uint128};

impl USDCEarnProgram {
    pub fn usdc_dummy(address: Option<String>, asset: String) -> USDCEarnProgram {
        match address {
            Some(s) => {
                return USDCEarnProgram {
                    bonding_period: 90,
                    apr: AprUsdc {
                        uusdc: 70,
                        ueden: 80,
                    },
                    available: Some(BalanceAvailable{
                        amount: 1400,
                        usd_amount: Decimal::from_atomics(Uint128::new(1400), 0).unwrap(),
                    }),
                    staked: Some(BalanceAvailable{
                        amount: 700,
                        usd_amount: Decimal::from_atomics(Uint128::new(700), 0).unwrap(),
                    }),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: "uusdc".to_string(),
                            amount: 900,
                            usd_amount: Some(Decimal::from_atomics(Uint128::new(900), 0).unwrap()),
                        },
                        BalanceReward {
                            asset: "ueden".to_string(),
                            amount: 1200,
                            usd_amount: Some(Decimal::from_atomics(Uint128::new(1200), 0).unwrap()),
                        },
                    ]),
                    borrowed: Some(BalanceBorrowed{
                        usd_amount: Decimal::from_atomics(Uint128::new(800), 0).unwrap(),
                        percentage: Decimal::from_atomics(Uint128::new(3), 0).unwrap(),
                    }),
                }
            },
            None => {
                return USDCEarnProgram {
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
        }
    }
}

use crate::types::earn_program::eden_boost_earn::EdenBoostEarnProgram;
use crate::types::BalanceReward;

use cosmwasm_std::{Decimal, Uint128};

impl EdenBoostEarnProgram {
    pub fn edenb_dummy(address: Option<String>, asset: String) -> EdenBoostEarnProgram {
        match address {
            Some(s) => {
                return EdenBoostEarnProgram {
                    bonding_period: 90,
                    apr: 100,
                    available: Some(1000),
                    staked: Some(2000),
                    rewards: Some(vec![BalanceReward {
                        asset: "uusdc".to_string(),
                        amount: 1500,
                        usd_amount: Some(Decimal::from_atomics(Uint128::new(1500), 0).unwrap()),
                    }]),
                }
            },
            None => {
                return EdenBoostEarnProgram {
                    bonding_period: 90,
                    apr: 100,
                    available: None,
                    staked: None,
                    rewards: None,
                }
            }
        }
    }
}

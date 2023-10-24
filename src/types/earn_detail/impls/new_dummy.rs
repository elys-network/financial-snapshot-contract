use crate::types::earn_detail::earn_detail::{Apr, BalanceDollar, VestingDetail, EarnDetail};
use cosmwasm_std::{Decimal, Uint128};

impl EarnDetail {
    pub fn new_dummy() -> EarnDetail {
        EarnDetail {
            apr: Apr{uusdc: 8,ueden: 10, uedenb: 100},
            bonding_period: 0,
            available: vec![
                BalanceDollar {
                    asset: "ueden".to_string(),
                    amount: 10,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(10), 0).unwrap(),
                },
            ],
            staked: vec![
                BalanceDollar {
                    asset: "ueden".to_string(),
                    amount: 100,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                },
            ],
            vesting: vec![BalanceDollar {
                asset: "ueden".to_string(),
                amount: 10,
                amount_in_usd: Decimal::from_atomics(Uint128::new(10), 0).unwrap(),
            },],
            rewards: vec![
                BalanceDollar{
                    asset: "ueden".to_string(),
                    amount: 50,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(50), 0).unwrap(),
                },
                BalanceDollar{
                    asset: "uusdc".to_string(),
                    amount: 100,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                }
            ],
            vesting_details: vec![VestingDetail {
                total_vest: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:100, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap()
                },
                balance_vested: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:100, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap()
                },
                remaining_vest: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:0, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap()
                },
                remaining_time: 10,
            }],
            asset: "ueden".to_string(),
        }
    }

    pub fn new_dummy_all() -> EarnDetail {
        EarnDetail {
            apr: Apr{uusdc: 8,ueden: 10, uedenb: 100},
            bonding_period: 0,
            available: vec![
                BalanceDollar {
                    asset: "ueden".to_string(),
                    amount: 10,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(10), 0).unwrap(),
                },
                BalanceDollar {
                    asset: "uusdc".to_string(),
                    amount: 0,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
                }
            ],
            staked: vec![
                BalanceDollar {
                    asset: "ueden".to_string(),
                    amount: 100,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                },
                BalanceDollar {
                    asset: "uusdc".to_string(),
                    amount: 100,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                }
            ],
            vesting: vec![BalanceDollar {
                asset: "ueden".to_string(),
                amount: 10,
                amount_in_usd: Decimal::from_atomics(Uint128::new(10), 0).unwrap(),
            },
            BalanceDollar {
                asset: "uusdc".to_string(),
                amount: 50,
                amount_in_usd: Decimal::from_atomics(Uint128::new(50), 0).unwrap(),
            }],
            rewards: vec![
                BalanceDollar{
                    asset: "ueden".to_string(),
                    amount: 50,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(50), 0).unwrap(),
                },
                BalanceDollar{
                    asset: "uusdc".to_string(),
                    amount: 100,
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
                }
            ],
            vesting_details: vec![VestingDetail {
                total_vest: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:100, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap()
                },
                balance_vested: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:100, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap()
                },
                remaining_vest: BalanceDollar{
                    asset: "ueden".to_string(),
                    amount:0, 
                    amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap()
                },
                remaining_time: 10,
            }],
            asset: "".to_string(),
        }
    }
}

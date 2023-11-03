use crate::types::earn_program::elys_earn::ElysEarnProgram;
use crate::types::{BalanceReward, AprElys, BalanceAvailable, StakedPosition, UnstakedPosition, StakingValidator};

use cosmwasm_std::{Decimal, Uint128};

impl ElysEarnProgram {
    pub fn elys_dummy(address: Option<String>, _asset: String) -> ElysEarnProgram {
        match address {
            Some(_s) => {
                return ElysEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: 70,
                        ueden: 80,
                        uedenb: 100,
                    },
                    available: Some(BalanceAvailable{
                        amount: 1000,
                        usd_amount: Decimal::from_atomics(Uint128::new(1000), 0).unwrap(),
                    }),
                    staked: Some(BalanceAvailable{
                        amount: 500,
                        usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                    }),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: "uusdc".to_string(),
                            amount: 800,
                            usd_amount: Some(Decimal::from_atomics(Uint128::new(800), 0).unwrap()),
                        },
                        BalanceReward {
                            asset: "ueden".to_string(),
                            amount: 1000,
                            usd_amount: Some(Decimal::from_atomics(Uint128::new(1000), 0).unwrap()),
                        },
                        BalanceReward {
                            asset: "uedenb".to_string(),
                            amount: 500,
                            usd_amount: None,
                        },
                    ]),
                    staked_positions: Some(
                        vec![
                            StakedPosition{
                                id: "10".to_string(),
                                validator: StakingValidator {
                                    address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                                    name: "validator15".to_string(),
                                    voting_power: Decimal::from_atomics(Uint128::new(40), 2).unwrap(),
                                    comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                                    profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                                },
                                staked: BalanceAvailable{
                                    amount: 1000,
                                    usd_amount: Decimal::from_atomics(Uint128::new(1000), 0).unwrap(),
                                },
                            }
                        ]
                    ),
                    unstaked_positions:Some(
                        vec![
                            UnstakedPosition{
                                id: "11".to_string(),
                                validator: StakingValidator {
                                    address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                                    name: "validator12".to_string(),
                                    voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                                    comission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
                                    profile_picture_src: None,
                                },
                                remaining_time: 1700074130000,
                                unstaked: BalanceAvailable{
                                    amount: 500,
                                    usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                                },
                            }
                        ]
                    ),
                }
            },
            None => {
                return ElysEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: 70,
                        ueden: 80,
                        uedenb: 100,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                    staked_positions: None,
                    unstaked_positions: None,
                }
            }
        }
    }
}

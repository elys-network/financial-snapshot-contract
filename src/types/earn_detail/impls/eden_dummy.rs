use crate::types::earn_program::eden_earn::EdenEarnProgram;
use crate::types::{BalanceReward, AprElys, BalanceAvailable, VestingDetail};

use cosmwasm_std::{Decimal, Uint128};

impl EdenEarnProgram {
    pub fn eden_dummy(address: Option<String>, asset: String) -> EdenEarnProgram {
        match address {
            Some(_s) => {
                return EdenEarnProgram {
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
                            asset: asset,
                            amount: 1000,
                            usd_amount: Some(Decimal::from_atomics(Uint128::new(1000), 0).unwrap()),
                        },
                        BalanceReward {
                            asset: "uedenb".to_string(),
                            amount: 500,
                            usd_amount: None,
                        },
                    ]),
                    vesting: Some(BalanceAvailable{
                        amount: 250,
                        usd_amount: Decimal::from_atomics(Uint128::new(250), 0).unwrap(),
                    }),
                    vesting_details: Some(vec![
                        VestingDetail{
                            id: "1".to_string(),
                            total_vest: BalanceAvailable{
                                amount: 500,
                                usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                            },
                            balance_vested: BalanceAvailable{
                                amount: 250,
                                usd_amount: Decimal::from_atomics(Uint128::new(250), 0).unwrap(),
                            },
                            remaining_vest: BalanceAvailable{
                                amount: 250,
                                usd_amount: Decimal::from_atomics(Uint128::new(250), 0).unwrap(),
                            },
                            remaining_time: 1701370130000,
                        },
                        VestingDetail{
                            id: "2".to_string(),
                            total_vest: BalanceAvailable{
                                amount: 800,
                                usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                            },
                            balance_vested: BalanceAvailable{
                                amount: 400,
                                usd_amount: Decimal::from_atomics(Uint128::new(250), 0).unwrap(),
                            },
                            remaining_vest: BalanceAvailable{
                                amount: 400,
                                usd_amount: Decimal::from_atomics(Uint128::new(250), 0).unwrap(),
                            },
                            remaining_time: 1703962130000,
                        }]
                    ),
                }
            },
            None => {
                return EdenEarnProgram {
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
    }
}

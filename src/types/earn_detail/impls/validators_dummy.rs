use crate::types::{ValidatorDetail, BalanceAvailable};
use crate::types::earn_program::ListValidators;

use cosmwasm_std::{Decimal, Uint128};

impl ListValidators {
    pub fn validators_dummy(name: Option<String>, address: Option<String>) -> Vec<ValidatorDetail> {
        match name {
            Some(n) => {
                match address {
                    Some(_addr) => {
                        vec![ValidatorDetail{
                            id: "1".to_string(),
                            name: n,
                            voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                            comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                            profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                            staked: Some(BalanceAvailable{
                                amount: 500,
                                usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                            }),
                        }]
                    },
                    None => {
                        vec![ValidatorDetail{
                            id: "1".to_string(),
                            name: n,
                            voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                            comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                            profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                            staked: None,
                        }]
                    }
                }
            },
            None => { 
                match address {
                    Some(_addr) => {
                        vec![
                            ValidatorDetail{
                                id: "1".to_string(),
                                name: "validator1".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(40), 2).unwrap(),
                                comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                                profile_picture_src: None,
                                staked: Some(BalanceAvailable{
                                    amount: 500,
                                    usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                                }),
                            },
                            ValidatorDetail{
                                id: "2".to_string(),
                                name: "validator2".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                                comission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
                                profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                                staked: Some(BalanceAvailable{
                                    amount: 2500,
                                    usd_amount: Decimal::from_atomics(Uint128::new(2500), 0).unwrap(),
                                }),
                            }
                        ]
                    },
                    None => {
                        vec![
                            ValidatorDetail{
                                id: "1".to_string(),
                                name: "validator1".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(40), 2).unwrap(),
                                comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                                profile_picture_src: None,
                                staked: None,
                            },
                            ValidatorDetail{
                                id: "2".to_string(),
                                name: "validator2".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                                comission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
                                profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                                staked: None,
                            }
                        ]
                    }
                }
            },
        }
    }
}
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
                            address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                            name: n,
                            voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                            commission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                            profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                            staked: Some(BalanceAvailable{
                                amount: 500,
                                usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                            }),
                        }]
                    },
                    None => {
                        vec![ValidatorDetail{
                            address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                            name: n,
                            voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                            commission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
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
                                address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                                name: "validator1".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(40), 2).unwrap(),
                                commission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                                profile_picture_src: None,
                                staked: Some(BalanceAvailable{
                                    amount: 500,
                                    usd_amount: Decimal::from_atomics(Uint128::new(500), 0).unwrap(),
                                }),
                            },
                            ValidatorDetail{
                                address: "elysvalcons1a86grpk3avnkn64gv7v4hx7ewej6qguut9654y".to_string(),
                                name: "validator2".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                                commission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
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
                                address: "elysvalcons1l9jv5sfe7qz6tunuhguzly8eutpvw3llzc3jx4".to_string(),
                                name: "validator1".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(40), 2).unwrap(),
                                commission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                                profile_picture_src: None,
                                staked: None,
                            },
                            ValidatorDetail{
                                address: "elysvalcons1a86grpk3avnkn64gv7v4hx7ewej6qguut9654y".to_string(),
                                name: "validator2".to_string(),
                                voting_power: Decimal::from_atomics(Uint128::new(60), 2).unwrap(),
                                commission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
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
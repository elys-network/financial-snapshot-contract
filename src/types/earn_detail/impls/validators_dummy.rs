use crate::types::ValidatorDetail;
use crate::types::earn_program::ListValidators;

use cosmwasm_std::{Decimal, Uint128};

impl ListValidators {
    pub fn validators_dummy(name: Option<String>) -> Vec<ValidatorDetail> {
        match name {
            Some(n) => {
                vec![ValidatorDetail{
                    id: "1".to_string(),
                    name: n,
                    voting_power: 9999,
                    comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                    profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                }]
            },
            None => { 
                vec![
                    ValidatorDetail{
                        id: "1".to_string(),
                        name: "validator1".to_string(),
                        voting_power: 9999,
                        comission: Decimal::from_atomics(Uint128::new(3), 2).unwrap(),
                        profile_picture_src: None,
                    },
                    ValidatorDetail{
                        id: "2".to_string(),
                        name: "validator2".to_string(),
                        voting_power: 6666,
                        comission: Decimal::from_atomics(Uint128::new(5), 2).unwrap(),
                        profile_picture_src: Some("https://i.pravatar.cc/300".to_string()),
                    }
                ]
            },
        }
    }
}
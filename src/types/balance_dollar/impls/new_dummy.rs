use crate::types::balance_dollar::balance_dollar::BalanceDollar;
use cosmwasm_std::{Decimal, Uint128};

impl BalanceDollar {
    pub fn new_dummy() -> BalanceDollar {
        BalanceDollar {
            asset: "uatom".to_string(),
            amount: 100,
            amount_in_usd: Decimal::from_atomics(Uint128::new(100), 0).unwrap(),
        }
    }

}

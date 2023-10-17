use crate::types::balance_dollar::balance_dollar::BalanceDollar;
use cosmwasm_std::{Decimal, Uint128};

impl BalanceDollar {
    pub fn init() -> BalanceDollar {
        BalanceDollar {
            asset: "".to_string(),
            amount: 0,
            amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
        }
    }
}

use crate::types::{vesting_detail::vesting_detail::VestingDetail, balance_dollar::balance_dollar::BalanceDollar};
use cosmwasm_std::{Decimal, Uint128};

impl VestingDetail {
    pub fn init() -> VestingDetail {
        VestingDetail {
            total_vest: BalanceDollar{
                asset: "".to_string(),
                amount:0, 
                amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap()
            },
            balance_vested: BalanceDollar{
                asset: "".to_string(),
                amount:0, 
                amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap()
            },
            remaining_vest: BalanceDollar{
                asset: "".to_string(),
                amount:0, 
                amount_in_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap()
            },
            remaining_time: 0,
        }
    }
}

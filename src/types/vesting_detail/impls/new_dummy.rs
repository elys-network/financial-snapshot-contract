use crate::types::{vesting_detail::vesting_detail::VestingDetail, balance_dollar::balance_dollar::BalanceDollar};
use cosmwasm_std::{Decimal, Uint128};

impl VestingDetail {
    pub fn new_dummy() -> VestingDetail {
        VestingDetail {
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
        }
    }

    pub fn new_dummys() -> Vec<VestingDetail> {
        vec![VestingDetail {
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
        }]
    }
}

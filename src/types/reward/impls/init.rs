use crate::types::reward::reward::Reward;
use cosmwasm_std::{Decimal, Uint128};

impl Reward {
    pub fn init() -> Reward {
        Reward {
            unclaimed_usdc_usd: Decimal::from_atomics(Uint128::new(0), 1).unwrap(),
            unclaimed_eden_usd: Decimal::from_atomics(Uint128::new(0), 1).unwrap(),
            unclaimed_eden_boost: 0,
            external_rewards_usd: Decimal::from_atomics(Uint128::new(0), 1).unwrap(),
        }
    }
}

use crate::types::liquid_asset::liquid_asset::LiquidAsset;
use cosmwasm_std::{Decimal, Uint128};

impl LiquidAsset {
    pub fn init() -> LiquidAsset {
        LiquidAsset {
            asset: "".to_string(),
            change_percent_24hr: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            total_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            total_token: 0,
            available_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            available_token: 0,
            in_order_usd: Decimal::from_atomics(Uint128::new(0), 0).unwrap(),
            in_order_token: 0,
        }
    }
}

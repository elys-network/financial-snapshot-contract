use crate::types::{liquid_asset::liquid_asset::LiquidAsset};

impl LiquidAsset {
    pub fn new_dummy() -> LiquidAsset {
        LiquidAsset {
            asset: "atom",
            change_percent_24hr: 5.2,
            total_usd: 100.0,
            total_token: 100,
            available_usd: 100.0,
            available_token: 0,
            in_order_usd: 0.0,
            in_order_token: 0,
        }
    }
}

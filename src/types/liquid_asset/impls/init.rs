use crate::types::{liquid_asset::liquid_asset::LiquidAsset};

impl LiquidAsset {
    pub fn init() -> LiquidAsset {
        LiquidAsset {
            asset: "",
            change_percent_24hr: 0.0,
            total_usd: 0.0,
            total_token: 0,
            available_usd: 0.0,
            available_token: 0,
            in_order_usd: 0.0,
            in_order_token: 0,
        }
    }
}

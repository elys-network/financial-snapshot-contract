use crate::types::{reward::reward::Reward};

impl Reward {
    pub fn init() -> Reward {
        Reward {
            unclaimed_usdc_usd: 0.0,
            unclaimed_eden_usd: 0.0,
            unclaimed_eden_boost: 0,
            external_rewards_usd: 0.0,
        }
    }
}

use crate::types::{reward::reward::Reward};

impl Reward {
    pub fn new_dummy() -> Reward {
        Reward {
            unclaimed_usdc_usd: 100.0,
            unclaimed_eden_usd: 100.0,
            unclaimed_eden_boost: 0,
            external_rewards_usd: 0.0,
        }
    }
}

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Reward {
    pub unclaimed_usdc_usd: u64,
    pub unclaimed_eden_usd: u64,
    pub unclaimed_eden_boost: u64,
    pub external_rewards_usd: u64,
}

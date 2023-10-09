use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct LiquidAsset {
    pub asset: String,
    pub change_percent_24hr: f64,
    pub total_usd: f64,
    pub total_token: u64,
    pub available_usd: f64,
    pub available_token: u64,
    pub in_order_usd: f64,
    pub in_order_token: u64,
}

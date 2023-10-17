use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct BalanceDollar {
    pub asset: String,
    pub amount: u64,
    pub amount_in_usd: Decimal,
}

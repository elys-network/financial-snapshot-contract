use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetBondingPeriodResp {
    pub bonding_period: u64,
}

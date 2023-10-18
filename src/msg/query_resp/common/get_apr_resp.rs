use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetAPRResp {
    pub apr: u64,
}

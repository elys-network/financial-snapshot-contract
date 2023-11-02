use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    StakeRequest {
        address: String,
        amount: u64,
        asset: String,
        validator_address: Option<String>,
    },
}

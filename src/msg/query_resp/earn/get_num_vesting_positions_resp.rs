use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct GetNumVestingPositionsResp {
    pub num_vesting_positions: u64,
}

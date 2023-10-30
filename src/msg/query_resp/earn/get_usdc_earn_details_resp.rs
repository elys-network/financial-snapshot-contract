use cosmwasm_schema::cw_serde;
use crate::types::earn_program::usdc_earn::USDCEarnProgram;

#[cw_serde]
pub struct GetUSDCEarnProgramResp {
    pub data: USDCEarnProgram,
}

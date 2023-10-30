use cosmwasm_schema::cw_serde;
use crate::types::earn_program::list_validator::ListOfValidators;

#[cw_serde]
pub struct GetListOfValidatorsResp {
    pub data: ListOfValidators,
}

use cosmwasm_schema::cw_serde;
use crate::types::ValidatorDetail;

#[cw_serde]
pub struct GetListOfValidatorsResp {
    pub validators: Vec<ValidatorDetail>,
}

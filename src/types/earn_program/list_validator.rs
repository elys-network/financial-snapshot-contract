use crate::types::ValidatorDetail;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ListOfValidators {
    pub validator: Vec<ValidatorDetail>,
}

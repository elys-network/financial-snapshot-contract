use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::earn::GetListOfValidatorsResp};
use crate::types::earn_program::ListValidators;

pub fn get_list_validators(_deps: Deps<ElysQuery>, name: Option<String>, address: Option<String>) -> Result<GetListOfValidatorsResp, ContractError> {
    // TODO:
    // Real backend logic implementation
    let resp = GetListOfValidatorsResp {
        validators: ListValidators::validators_dummy(name, address)
    };

    Ok(resp)
}
use crate::bindings::query::ElysQuery;

use super::*;

pub fn stake_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    // the address of the current user.
    _address: String,
    // the amount to be staked in base denomination.
    _amount: u64,
    // The asset to be staked
    _asset: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    _validator_address: Option<String>
) -> Result<Response<ElysMsg>, ContractError> {
    let resp = Response::new().add_message(CosmosMsg::Bank(refund_msg));

    Ok(resp)
}
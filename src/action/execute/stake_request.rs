use crate::{bindings::query::ElysQuery, bindings::querier::ElysQuerier};

use super::*;

pub fn stake_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    // the address of the current user.
    address: String,
    // the amount to be staked in base denomination.
    _amount: u64,
    // The asset to be staked
    asset: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    _validator_address: Option<String>
) -> Result<Response<ElysMsg>, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let balance = querier.get_balance(address.to_owned(), asset.to_owned())?;
    // let resp: QueryBalanceResponse = QueryBalanceResponse { balance };

    let msg = BankMsg::Send {
        to_address: address.to_owned(),
        amount: vec![balance],
    };

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
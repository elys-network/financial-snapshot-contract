use crate::bindings::query::ElysQuery;

use super::*;
pub fn claim_rewards_request(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut<ElysQuery>,
    delegator_address: String,
    denom: String,
) -> Result<Response<ElysMsg>, ContractError> {
    let msg = ElysMsg::withdraw_rewards(
        delegator_address,
        denom,
    );

    let resp = Response::new().add_message(msg);

    Ok(resp)
}
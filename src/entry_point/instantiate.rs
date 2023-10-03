use super::*;
use crate::{bindings::query::ElysQuery};
use msg::InstantiateMsg;

pub fn instantiate(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

use super::*;
use msg::ExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<ElysQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    match msg {
    }
}

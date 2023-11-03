use super::*;
use msg::ExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    use action::execute::*;
    use ExecuteMsg::*;

    match msg {
        StakeRequest { address, amount, asset, validator_address } => stake_request(env, info, deps, address, amount, asset, validator_address),
    }
}

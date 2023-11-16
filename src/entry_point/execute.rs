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
        UnstakeRequest { address, amount, asset, validator_address } => unstake_request(env, info, deps, address, amount, asset, validator_address),
        ElysRedelegateRequest { delegator_address, validator_src_address, validator_dst_address, amount} => elys_redelegation_request(env, info, deps, delegator_address, validator_src_address, validator_dst_address, amount),
        ElysCancelUnstakeRequest { delegator_address, validator_address, amount, creation_height } => elys_cancel_unstake_request(env, info, deps, delegator_address, validator_address, amount, creation_height),
        EdenVestRequest { creator, amount} => eden_vest_request(env, info, deps, creator, amount),
        EdenCancelVestRequest { creator, amount  } => eden_cancel_vest_request(env, info, deps, creator, amount),
        ClaimRewardsRequest { delegator_address, denom, withdraw_type } => claim_rewards_request(env, info, deps, delegator_address, denom, withdraw_type),
        ClaimValidatorCommissionRequest { delegator_address, validator_address, denom } => claim_validator_commission_request(env, info, deps, delegator_address, validator_address, denom),
    }
}

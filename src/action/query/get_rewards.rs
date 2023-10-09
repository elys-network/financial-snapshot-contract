use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetRewardsResp, types::*};

pub fn get_rewards(deps: Deps<ElysQuery>, address: String) -> Result<GetRewardsResp, ContractError> {
    let rewards: Reward = REWARDS.load(deps.storage, &address)?;
    let resp = GetRewardsResp {
        rewards: rewards,
    };

    Ok(resp)
}

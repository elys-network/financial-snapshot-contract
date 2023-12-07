use super::*;
use crate::{bindings::{querier::ElysQuerier, query::ElysQuery}, msg::query_resp::earn::{QueryPoolResponse, FilterType}};
use crate::types::PageRequest;

pub fn get_pools(deps: Deps<ElysQuery>, pool_ids: Option<Vec<u64>>, filter_type: FilterType, pagination: Option<PageRequest>) -> Result<QueryPoolResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let pools = querier.get_all_pools()?;

    let filtered_pools = match filter_type {
        FilterType::FilterAll => pools.clone(),
        FilterType::FilterPerpetual => pools
        .iter()
        .cloned()
        .collect(),
        FilterType::FilterFixedWeight => pools
        .iter()
        .filter(|p| p.pool_params.use_oracle == true)
        .cloned()
        .collect(),
        FilterType::FilterDynamicWeight =>pools
        .iter()
        .filter(|p| p.pool_params.use_oracle == false)
        .cloned()
        .collect(),
    };
    let resp: QueryPoolResponse;

    Ok(resp)
}
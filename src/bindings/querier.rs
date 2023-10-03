#[allow(unused_imports)]
use cosmwasm_std::{QuerierWrapper};
use super::query::ElysQuery;

#[allow(dead_code)]
pub struct ElysQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ElysQuery>,
}

#[allow(dead_code)]
impl<'a> ElysQuerier<'a> {
}

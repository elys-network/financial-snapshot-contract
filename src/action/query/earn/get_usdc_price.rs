use super::*;
use cosmwasm_std::{Decimal, Uint128};
use crate::bindings::{query::ElysQuery, querier::ElysQuerier};
use crate::types::ElysDenom;

pub fn get_usdc_price(deps: Deps<ElysQuery>) -> Result<Decimal, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);
    let usdc_oracle_price = querier.get_oracle_price(ElysDenom::USDC.as_str().to_string(), "".to_string(), 0)?;
    let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
    Ok(usdc_usd_price)
}
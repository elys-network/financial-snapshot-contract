use crate::types::BalanceDollar;
use cw_storage_plus::Map;

pub const BALANCE_STAKED: Map<&str, Vec<BalanceDollar>> = Map::new("balance_staked");

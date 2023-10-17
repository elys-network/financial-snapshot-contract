use crate::types::BalanceDollar;
use cw_storage_plus::Map;

pub const BALANCE_AVAILABLE: Map<&str, Vec<BalanceDollar>> = Map::new("balance_available");

use crate::types::BalanceDollar;
use cw_storage_plus::Map;

pub const BALANCE_VESTING: Map<&str, Vec<BalanceDollar>> = Map::new("balance_vesting");

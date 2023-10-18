use crate::types::VestingDetail;
use cw_storage_plus::Map;

pub const VESTING_DETAILS: Map<&str, Vec<VestingDetail>> = Map::new("vesting_details");

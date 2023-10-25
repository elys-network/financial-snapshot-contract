use crate::types::EarnDetail;
use cw_storage_plus::Map;

pub const EARN_DETAILS: Map<&str, EarnDetail> = Map::new("earn_details");

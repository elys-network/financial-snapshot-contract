use crate::{
    entry_point::{execute, query},
    msg::*,
    ContractError,
};

mod get_portfolio {
    use super::*;
    use cosmwasm_std::{Binary, StdError};
    mod not_found;
    mod successful_query_message;
}

pub use mock::instantiate::*;
mod mock {
    #[allow(dead_code, unused)]
    pub mod instantiate;
}

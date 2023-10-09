use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
    #[error("{liquid_asset} : Not Found")]
    LiquidAssetNotFound { liquid_asset: String },
}

use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
    #[error("{address} : Not Found")]
    PortfolioNotFound { address: String },
    #[error("{sender} is not the owner of the order")]
    Unauthorized { sender: Addr },
}

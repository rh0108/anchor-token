use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not Implement")]
    NotImplement {},

    #[error("Gauge Not Found")]
    GaugeNotFound {},

    #[error("Gauge Already Exist")]
    GaugeAlreadyExist {},
}

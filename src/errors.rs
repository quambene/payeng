use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DepositError {
    #[error("Can't deposit transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't deposit transaction: invalid format for amount")]
    InvalidFormat,
}

#[derive(Error, Debug)]
pub enum WithdrawalError {
    #[error("Can't withdraw transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't withdraw transaction: insufficient funds")]
    InsufficientFunds,
    #[error("Can't withdraw transaction: invalid format for amount")]
    InvalidFormat,
}

#[derive(Error, Debug)]
pub enum DisputeError {
    #[error("Can't dispute transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't dispute transaction: invalid format for amount")]
    InvalidFormat,
}

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("Can't resolve transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't resolve transaction: invalid format for amount")]
    InvalidFormat,
}

#[derive(Error, Debug)]
pub enum ChargebackError {
    #[error("Can't chargeback transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't chargeback transaction: invalid format for amount")]
    InvalidFormat,
}

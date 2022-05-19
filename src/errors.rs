use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DepositError {
    #[error("Can't deposit transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't deposit transaction: invalid transaction type")]
    InvalidTransactionType,
}

#[derive(Error, Debug)]
pub enum WithdrawalError {
    #[error("Can't withdraw transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't withdraw transaction: invalid transaction type")]
    InvalidTransactionType,
    #[error("Can't withdraw transaction: insufficient funds")]
    InsufficientFunds,
}

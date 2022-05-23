use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FormatError {
    #[error(
        "Unexpected format: invalid transaction type '{}' in transaction id {}",
        .0, .1
    )]
    InvalidTransactionType(String, u32),
    #[error("Unexpected format: missing amount for transaction id {} and transaction type '{}'", .0, .1)]
    MissingAmount(u32, String),
    #[error("Unexpected format: amount should be none for transaction id {} and transaction type '{}'", .0, .1)]
    UnexpectedAmount(u32, String),
    #[error("Unexpected format: amount is negative, infinite or NaN for transaction id {} and transaction type '{}'", .0, .1)]
    InvalidAmount(u32, String),
}

#[derive(Error, Debug, PartialEq)]
pub enum DepositError {
    #[error("Can't deposit transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't deposit transaction: account is frozen for client id {}", .0)]
    FrozenAccount(u16),
    #[error("Can't deposit transaction: invalid transaction type for transaction id {}", .0)]
    InvalidTransactionType(u32),
}

#[derive(Error, Debug, PartialEq)]
pub enum WithdrawalError {
    #[error("Can't withdraw transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't withdraw transaction: insufficient funds for client id {}", .0)]
    InsufficientFunds(u16),
    #[error("Can't withdraw transaction: account is frozen for client id {}", .0)]
    FrozenAccount(u16),
    #[error("Can't withdraw transaction: invalid transaction type for transaction id {}", .0)]
    InvalidTransactionType(u32),
}

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
    #[error("Unexpected format: transaction id {} is not unique", .0)]
    UniqueTransactionId(u32),
}

#[derive(Error, Debug, PartialEq)]
pub enum DepositError {
    // TODO: improve error message and error name for InvalidClientId
    #[error("Can't deposit transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't deposit transaction: account is frozen for client id {}", 0.)]
    FrozenAccount(u16),
}

#[derive(Error, Debug, PartialEq)]
pub enum WithdrawalError {
    // TODO: improve error message and error name for InvalidClientId
    #[error("Can't withdraw transaction: invalid client id")]
    InvalidClientId,
    #[error("Can't withdraw transaction: insufficient funds for client id {}", .0)]
    InsufficientFunds(u16),
    #[error("Can't withdraw transaction: account is frozen for client id {}", 0.)]
    FrozenAccount(u16),
}

#[derive(Error, Debug)]
pub enum DisputeError {
    // TODO: improve error message and error name for InvalidClientId
    #[error("Can't dispute transaction: invalid client id")]
    InvalidClientId,
}

#[derive(Error, Debug)]
pub enum ResolveError {
    // TODO: improve error message and error name for InvalidClientId
    #[error("Can't resolve transaction: invalid client id")]
    InvalidClientId,
}

#[derive(Error, Debug)]
pub enum ChargebackError {
    // TODO: improve error message and error name for InvalidClientId
    #[error("Can't chargeback transaction: invalid client id")]
    InvalidClientId,
}

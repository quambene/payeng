use super::{round, Transaction};
use crate::{
    errors::FormatError,
    models::{CheckedTransaction, EventType, TransactionEvent, TransactionType},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    pub r#type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

// Used in tests
#[allow(dead_code)]
impl RawTransaction {
    pub fn new(r#type: String, client: u16, tx: u32, amount: Option<f64>) -> Self {
        Self {
            r#type,
            client,
            tx,
            amount,
        }
    }
}

impl TryFrom<RawTransaction> for CheckedTransaction {
    type Error = FormatError;

    fn try_from(tx: RawTransaction) -> Result<CheckedTransaction, Self::Error> {
        match &tx.r#type {
            x if x == "deposit" => Ok(CheckedTransaction::Transaction(Transaction::new(
                TransactionType::Deposit,
                tx.client,
                tx.tx,
                validate(&tx, x)?,
            ))),
            x if x == "withdrawal" => Ok(CheckedTransaction::Transaction(Transaction::new(
                TransactionType::Withdrawal,
                tx.client,
                tx.tx,
                validate(&tx, x)?,
            ))),
            x if x == "dispute" => {
                match tx.amount {
                    Some(_) => return Err(FormatError::UnexpectedAmount(tx.tx, x.to_string())),
                    None => (),
                }

                Ok(CheckedTransaction::TransactionEvent(TransactionEvent::new(
                    EventType::Dispute,
                    tx.client,
                    tx.tx,
                )))
            }
            x if x == "resolve" => {
                match tx.amount {
                    Some(_) => return Err(FormatError::UnexpectedAmount(tx.tx, x.to_string())),
                    None => (),
                }

                Ok(CheckedTransaction::TransactionEvent(TransactionEvent::new(
                    EventType::Resolve,
                    tx.client,
                    tx.tx,
                )))
            }
            x if x == "chargeback" => {
                match tx.amount {
                    Some(_) => return Err(FormatError::UnexpectedAmount(tx.tx, x.to_string())),
                    None => (),
                }

                Ok(CheckedTransaction::TransactionEvent(TransactionEvent::new(
                    EventType::Chargeback,
                    tx.client,
                    tx.tx,
                )))
            }
            x => {
                return Err(FormatError::InvalidTransactionType(x.to_string(), tx.tx));
            }
        }
    }
}

fn validate(tx: &RawTransaction, transaction_type: &str) -> Result<f64, FormatError> {
    match tx.amount {
        Some(amount) => {
            if amount.is_finite() && amount.is_sign_positive() {
                Ok(round(amount))
            } else {
                Err(FormatError::InvalidAmount(
                    tx.tx,
                    transaction_type.to_string(),
                ))
            }
        }
        None => {
            return Err(FormatError::MissingAmount(
                tx.tx,
                transaction_type.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transaction() {
        let raw_transaction = RawTransaction::new("deposit".to_string(), 1, 1, Some(25.0));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_ok());
    }

    #[test]
    fn test_invalid_transaction_type() {
        let raw_transaction = RawTransaction::new("unknown".to_string(), 1, 1, Some(25.0));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(
            err,
            FormatError::InvalidTransactionType("unknown".to_string(), 1)
        );
    }

    #[test]
    fn test_missing_amount() {
        let raw_transaction = RawTransaction::new("deposit".to_string(), 1, 1, None);

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::MissingAmount(1, "deposit".to_string()));
    }

    #[test]
    fn test_unexpected_amount() {
        let raw_transaction = RawTransaction::new("dispute".to_string(), 1, 1, Some(25.0));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::UnexpectedAmount(1, "dispute".to_string()));
    }

    #[test]
    fn test_infinity_amount() {
        let raw_transaction = RawTransaction::new("deposit".to_string(), 1, 1, Some(f64::INFINITY));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::InvalidAmount(1, "deposit".to_string()));
    }

    #[test]
    fn test_nan_amount() {
        let raw_transaction = RawTransaction::new("deposit".to_string(), 1, 1, Some(f64::NAN));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::InvalidAmount(1, "deposit".to_string()));
    }

    #[test]
    fn test_neg_infinity_amount() {
        let raw_transaction =
            RawTransaction::new("deposit".to_string(), 1, 1, Some(f64::NEG_INFINITY));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::InvalidAmount(1, "deposit".to_string()));
    }

    #[test]
    fn test_negative_amount() {
        let raw_transaction = RawTransaction::new("deposit".to_string(), 1, 1, Some(-25.0));

        let res: Result<CheckedTransaction, FormatError> = raw_transaction.try_into();
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, FormatError::InvalidAmount(1, "deposit".to_string()));
    }
}

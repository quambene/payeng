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
            if amount.is_finite() {
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

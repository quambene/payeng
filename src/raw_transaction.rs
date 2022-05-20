use crate::transaction::TransactionType;

use super::Transaction;
use anyhow::anyhow;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    pub r#type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = anyhow::Error;

    fn try_from(tx: RawTransaction) -> Result<Transaction, Self::Error> {
        match &tx.r#type {
            x if x == "deposit" => Ok(Transaction::new(
                TransactionType::Deposit,
                tx.client,
                tx.tx,
                tx.amount,
            )),
            x if x == "withdrawal" => Ok(Transaction::new(
                TransactionType::Withdrawal,
                tx.client,
                tx.tx,
                tx.amount,
            )),
            x if x == "dispute" => Ok(Transaction::new(
                TransactionType::Dispute,
                tx.client,
                tx.tx,
                tx.amount,
            )),
            x if x == "resolve" => Ok(Transaction::new(
                TransactionType::Resolve,
                tx.client,
                tx.tx,
                tx.amount,
            )),
            x if x == "chargeback" => Ok(Transaction::new(
                TransactionType::Chargeback,
                tx.client,
                tx.tx,
                tx.amount,
            )),
            x => {
                return Err(anyhow!("Transaction type '{}' not supported", x));
            }
        }
    }
}

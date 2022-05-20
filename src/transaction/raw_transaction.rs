use super::{DepositTransaction, Transaction, WithdrawalTransaction};
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
            x if x == "deposit" => Ok(Transaction::Deposit(DepositTransaction::new(
                tx.client,
                tx.tx,
                tx.amount.unwrap(),
            ))),
            x if x == "withdrawal" => Ok(Transaction::Withdrawal(WithdrawalTransaction::new(
                tx.client,
                tx.tx,
                tx.amount.unwrap(),
            ))),
            x => {
                return Err(anyhow!("Transaction type '{}' not supported", x));
            }
        }
    }
}

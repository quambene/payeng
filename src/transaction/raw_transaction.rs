use super::{
    ChargebackTransaction, DepositTransaction, DisputeTransaction, ResolveTransaction, Transaction,
    WithdrawalTransaction,
};
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
                match tx.amount {
                    Some(amount) => amount,
                    None => return Err(anyhow!("Invalid format for deposit transaction")),
                },
            ))),
            x if x == "withdrawal" => Ok(Transaction::Withdrawal(WithdrawalTransaction::new(
                tx.client,
                tx.tx,
                match tx.amount {
                    Some(amount) => amount,
                    None => return Err(anyhow!("Invalid format for withdrawal transaction")),
                },
            ))),
            x if x == "dispute" => match tx.amount {
                Some(_) => return Err(anyhow!("Invalid format for dispute transaction")),
                None => Ok(Transaction::Dispute(DisputeTransaction::new(
                    tx.client, tx.tx,
                ))),
            },
            x if x == "resolve" => match tx.amount {
                Some(_) => return Err(anyhow!("Invalid format for resolve transaction")),
                None => Ok(Transaction::Resolve(ResolveTransaction::new(
                    tx.client, tx.tx,
                ))),
            },
            x if x == "chargeback" => match tx.amount {
                Some(_) => return Err(anyhow!("Invalid format for chargeback transaction")),
                None => Ok(Transaction::Chargeback(ChargebackTransaction::new(
                    tx.client, tx.tx,
                ))),
            },
            x => {
                return Err(anyhow!("Transaction type '{}' not supported", x));
            }
        }
    }
}

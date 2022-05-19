// TODO: check decimal precision
// TODO: implement new type pattern for client_id

use crate::transaction::{Transaction, TransactionType};
use anyhow::{anyhow, Error};

#[derive(Debug, PartialEq)]
pub struct Account {
    client_id: u16,
    available: f64,
    held: f64,
    total: f64,
    is_locked: bool,
}

// TODO: implement dispute, resolve
// TODO: consider using  std::Error or thiserror instead of anyhow
impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            is_locked: false,
        }
    }

    pub fn deposit(&mut self, tx: &Transaction) -> Result<(), Error> {
        match tx.tx_type {
            TransactionType::Deposit => {
                if tx.client_id == self.client_id {
                    self.available = self.available + tx.amount;
                    self.total = self.total + tx.amount;
                    Ok(())
                } else {
                    Err(anyhow!("Can't deposit transaction: invalid client id"))
                }
            }
            _ => Err(anyhow!(
                "Can't deposit transaction: invalid transaction type"
            )),
        }
    }

    pub fn withdraw(&mut self, tx: &Transaction) -> Result<(), Error> {
        match tx.tx_type {
            TransactionType::Withdrawal => {
                if tx.client_id == self.client_id {
                    self.available = self.available - tx.amount;
                    self.total = self.total - tx.amount;
                    Ok(())
                } else {
                    Err(anyhow!("Can't withdraw transaction: invalid client id"))
                }
            }
            _ => Err(anyhow!(
                "Can't withdraw transaction: invalid transaction type"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client_id = 42;
        let account = Account::new(client_id);
        assert_eq!(
            account,
            Account {
                client_id,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                is_locked: false
            }
        )
    }

    #[test]
    fn test_deposit() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Deposit, client_id, 1, 25.0);
        let res = account.deposit(&transaction);

        assert!(res.is_ok());
        assert_eq!(
            account,
            Account {
                client_id,
                available: 25.0,
                held: 0.0,
                total: 25.0,
                is_locked: false
            }
        );
    }

    #[test]
    fn test_deposit_invalid_transaction_type() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Withdrawal, client_id, 1, 25.0);
        let res = account.deposit(&transaction);

        assert!(res.is_err());
    }

    #[test]
    fn test_deposit_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = Transaction::new(TransactionType::Deposit, 2, 1, 25.0);
        let res = account.withdraw(&transaction);

        assert!(res.is_err());
    }

    #[test]
    fn test_withdraw() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Withdrawal, client_id, 1, 25.0);
        let res = account.withdraw(&transaction);

        assert!(res.is_ok());
        assert_eq!(
            account,
            Account {
                client_id,
                available: -25.0,
                held: 0.0,
                total: -25.0,
                is_locked: false
            }
        );
    }

    #[test]
    fn test_withdraw_invalid_transaction_type() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Deposit, client_id, 1, 25.0);
        let res = account.withdraw(&transaction);

        assert!(res.is_err());
    }

    #[test]
    fn test_withdraw_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = Transaction::new(TransactionType::Withdrawal, 2, 1, 25.0);
        let res = account.deposit(&transaction);

        assert!(res.is_err());
    }
}

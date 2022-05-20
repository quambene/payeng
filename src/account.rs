// TODO: check decimal precision
// TODO: implement new type pattern for client_id

use crate::{
    errors::{ChargebackError, DepositError, DisputeError, ResolveError, WithdrawalError},
    raw_account::RawAccount,
    transaction::Transaction,
};

#[derive(Debug, PartialEq)]
pub struct Account {
    pub client_id: u16,
    pub available_amount: f64,
    pub held_amount: f64,
    pub total_amount: f64,
    pub is_locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available_amount: 0.0,
            held_amount: 0.0,
            total_amount: 0.0,
            is_locked: false,
        }
    }

    pub fn deposit(&mut self, tx: &Transaction) -> Result<(), DepositError> {
        if self.client_id == tx.client_id {
            match tx.amount {
                Some(amount) => {
                    self.available_amount = self.available_amount + amount;
                    self.total_amount = self.total_amount + amount;
                    Ok(())
                }
                None => Err(DepositError::InvalidFormat),
            }
        } else {
            Err(DepositError::InvalidClientId)
        }
    }

    pub fn withdraw(&mut self, tx: &Transaction) -> Result<(), WithdrawalError> {
        if self.client_id == tx.client_id {
            match tx.amount {
                Some(amount) => {
                    if self.available_amount - amount >= 0.0 {
                        self.available_amount = self.available_amount - amount;
                        self.total_amount = self.total_amount - amount;
                        Ok(())
                    } else {
                        Err(WithdrawalError::InsufficientFunds)
                    }
                }
                None => Err(WithdrawalError::InvalidFormat),
            }
        } else {
            Err(WithdrawalError::InvalidClientId)
        }
    }

    pub fn dispute(&mut self, tx: &Transaction, txs: &[Transaction]) -> Result<(), DisputeError> {
        if self.client_id == tx.client_id {
            match tx.amount {
                Some(_amount) => Err(DisputeError::InvalidFormat),
                None => {
                    // let disputed_tx = txs.iter().find(|&&x| x.client_id == self.client_id)
                    todo!();
                }
            }
        } else {
            Err(DisputeError::InvalidClientId)
        }
    }

    pub fn resolve(&mut self, tx: &Transaction, txs: &[Transaction]) -> Result<(), ResolveError> {
        if self.client_id == tx.client_id {
            match tx.amount {
                Some(_amount) => Err(ResolveError::InvalidFormat),
                None => {
                    // let resolved_tx = txs.iter().find(|&&x| x.client_id == self.client_id)
                    todo!();
                }
            }
        } else {
            Err(ResolveError::InvalidClientId)
        }
    }

    pub fn chargeback(
        &mut self,
        tx: &Transaction,
        txs: &[Transaction],
    ) -> Result<(), ChargebackError> {
        if self.client_id == tx.client_id {
            match tx.amount {
                Some(_amount) => Err(ChargebackError::InvalidFormat),
                None => {
                    // let chargebacked_tx = txs.iter().find(|&&x| x.client_id == self.client_id)
                    todo!();
                }
            }
        } else {
            Err(ChargebackError::InvalidClientId)
        }
    }
}

impl From<Account> for RawAccount {
    fn from(account: Account) -> RawAccount {
        RawAccount {
            client: account.client_id,
            available: account.available_amount,
            held: account.held_amount,
            total: account.total_amount,
            locked: account.is_locked,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::TransactionType;

    use super::*;

    #[test]
    fn test_new() {
        let client_id = 42;
        let account = Account::new(client_id);
        assert_eq!(
            account,
            Account {
                client_id,
                available_amount: 0.0,
                held_amount: 0.0,
                total_amount: 0.0,
                is_locked: false
            }
        )
    }

    #[test]
    fn test_deposit() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Deposit, client_id, 1, Some(25.0));
        let res = account.deposit(&transaction);

        assert!(res.is_ok());
        assert_eq!(
            account,
            Account {
                client_id,
                available_amount: 25.0,
                held_amount: 0.0,
                total_amount: 25.0,
                is_locked: false
            }
        );
    }

    #[test]
    fn test_deposit_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = Transaction::new(TransactionType::Deposit, 2, 1, Some(25.0));

        let res = account.deposit(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, DepositError::InvalidClientId));
    }

    #[test]
    fn test_withdraw() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let deposit_transaction =
            Transaction::new(TransactionType::Deposit, client_id, 1, Some(25.0));
        let withdrawal_transaction =
            Transaction::new(TransactionType::Withdrawal, client_id, 1, Some(15.0));

        let res = account.deposit(&deposit_transaction);
        assert!(res.is_ok());

        let res = account.withdraw(&withdrawal_transaction);
        assert!(res.is_ok());

        assert_eq!(
            account,
            Account {
                client_id,
                available_amount: 10.0,
                held_amount: 0.0,
                total_amount: 10.0,
                is_locked: false
            }
        );
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let transaction = Transaction::new(TransactionType::Withdrawal, client_id, 1, Some(25.0));

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, WithdrawalError::InsufficientFunds));
    }

    #[test]
    fn test_withdraw_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = Transaction::new(TransactionType::Withdrawal, 2, 1, Some(15.0));

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, WithdrawalError::InvalidClientId));
    }
}

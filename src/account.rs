// TODO: check decimal precision
// TODO: implement new type pattern for client_id

use crate::{
    raw_account::RawAccount,
    transaction::{
        ChargebackError, ChargebackTransaction, DepositError, DepositTransaction, DisputeError,
        DisputeTransaction, ResolveError, ResolveTransaction, WithdrawalError,
        WithdrawalTransaction,
    },
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

    pub fn deposit(&mut self, tx: &DepositTransaction) -> Result<(), DepositError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount + tx.amount;
            self.total_amount = self.total_amount + tx.amount;
            Ok(())
        } else {
            Err(DepositError::InvalidClientId)
        }
    }

    pub fn withdraw(&mut self, tx: &WithdrawalTransaction) -> Result<(), WithdrawalError> {
        if self.client_id == tx.client_id {
            if self.available_amount - tx.amount >= 0.0 {
                self.available_amount = self.available_amount - tx.amount;
                self.total_amount = self.total_amount - tx.amount;
                Ok(())
            } else {
                Err(WithdrawalError::InsufficientFunds)
            }
        } else {
            Err(WithdrawalError::InvalidClientId)
        }
    }

    pub fn dispute(&mut self, tx: &DisputeTransaction) -> Result<(), DisputeError> {
        if self.client_id == tx.client_id {
            todo!()
        } else {
            Err(DisputeError::InvalidClientId)
        }
    }

    pub fn resolve(&mut self, tx: &ResolveTransaction) -> Result<(), ResolveError> {
        if self.client_id == tx.client_id {
            todo!()
        } else {
            Err(ResolveError::InvalidClientId)
        }
    }

    pub fn chargeback(&mut self, tx: &ChargebackTransaction) -> Result<(), ChargebackError> {
        if self.client_id == tx.client_id {
            todo!()
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
        let transaction = DepositTransaction::new(client_id, 1, 25.0);
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
        let transaction = DepositTransaction::new(2, 1, 25.0);

        let res = account.deposit(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, DepositError::InvalidClientId));
    }

    #[test]
    fn test_withdraw() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let deposit_transaction = DepositTransaction::new(client_id, 1, 25.0);
        let withdrawal_transaction = WithdrawalTransaction::new(client_id, 1, 15.0);

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
        let transaction = WithdrawalTransaction::new(client_id, 1, 25.0);

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, WithdrawalError::InsufficientFunds));
    }

    #[test]
    fn test_withdraw_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = WithdrawalTransaction::new(2, 1, 15.0);

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, WithdrawalError::InvalidClientId));
    }
}

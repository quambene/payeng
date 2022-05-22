use crate::{
    errors::{ChargebackError, DepositError, DisputeError, ResolveError, WithdrawalError},
    models::{
        round, RawAccount, {Transaction, TransactionType},
    },
};

// TODO: implement locked account
// TODO: implement new type pattern for client_id
#[derive(Debug, PartialEq)]
pub struct Account {
    pub client_id: u16,
    // available_amount should be positive
    pub available_amount: f64,
    // held amount can be negative
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

    pub fn freeze(&mut self) {
        self.is_locked = true;
    }

    pub fn deposit(&mut self, tx: &Transaction) -> Result<(), DepositError> {
        if self.client_id == tx.client_id {
            if !self.is_locked {
                self.available_amount = self.available_amount + tx.amount;
                self.total_amount = self.total_amount + tx.amount;
                Ok(())
            } else {
                // This aborts processing and is potentially undesirable
                Err(DepositError::FrozenAccount(self.client_id))
            }
        } else {
            // Abort processing as input data are seriously flawed
            Err(DepositError::InvalidClientId)
        }
    }

    pub fn withdraw(&mut self, tx: &Transaction) -> Result<(), WithdrawalError> {
        if self.client_id == tx.client_id {
            if !self.is_locked {
                if self.available_amount - tx.amount >= 0.0 {
                    self.available_amount = self.available_amount - tx.amount;
                    self.total_amount = self.total_amount - tx.amount;
                    Ok(())
                } else {
                    // This aborts processing and is potentially undesirable
                    Err(WithdrawalError::InsufficientFunds(self.client_id))
                }
            } else {
                // This aborts processing and is potentially undesirable
                Err(WithdrawalError::FrozenAccount(self.client_id))
            }
        } else {
            // Abort processing as input data are seriously flawed
            Err(WithdrawalError::InvalidClientId)
        }
    }

    pub fn dispute(&mut self, tx: &Transaction) -> Result<(), DisputeError> {
        match tx.transaction_type {
            TransactionType::Deposit => self.dispute_deposit(tx),
            TransactionType::Withdrawal => self.dispute_withdrawal(tx),
        }
    }

    pub fn resolve(&mut self, tx: &Transaction) -> Result<(), ResolveError> {
        match tx.transaction_type {
            TransactionType::Deposit => self.resolve_deposit(tx),
            TransactionType::Withdrawal => self.resolve_withdrawal(tx),
        }
    }

    pub fn chargeback(&mut self, tx: &Transaction) -> Result<(), ChargebackError> {
        match tx.transaction_type {
            TransactionType::Deposit => self.chargeback_deposit(tx),
            TransactionType::Withdrawal => self.chargeback_withdrawal(tx),
        }
    }

    fn dispute_deposit(&mut self, tx: &Transaction) -> Result<(), DisputeError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount - tx.amount;
            self.held_amount = self.held_amount + tx.amount;
            Ok(())
        } else {
            Err(DisputeError::InvalidClientId)
        }
    }

    fn dispute_withdrawal(&mut self, tx: &Transaction) -> Result<(), DisputeError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount + tx.amount;
            self.held_amount = self.held_amount - tx.amount;
            Ok(())
        } else {
            Err(DisputeError::InvalidClientId)
        }
    }

    fn resolve_deposit(&mut self, tx: &Transaction) -> Result<(), ResolveError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount + tx.amount;
            self.held_amount = self.held_amount - tx.amount;
            Ok(())
        } else {
            Err(ResolveError::InvalidClientId)
        }
    }

    fn resolve_withdrawal(&mut self, tx: &Transaction) -> Result<(), ResolveError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount - tx.amount;
            self.held_amount = self.held_amount + tx.amount;
            Ok(())
        } else {
            Err(ResolveError::InvalidClientId)
        }
    }

    fn chargeback_deposit(&mut self, tx: &Transaction) -> Result<(), ChargebackError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount - tx.amount;
            self.held_amount = self.held_amount + tx.amount;
            self.total_amount = self.total_amount + tx.amount;
            Ok(())
        } else {
            Err(ChargebackError::InvalidClientId)
        }
    }

    fn chargeback_withdrawal(&mut self, tx: &Transaction) -> Result<(), ChargebackError> {
        if self.client_id == tx.client_id {
            self.available_amount = self.available_amount + tx.amount;
            self.held_amount = self.held_amount - tx.amount;
            self.total_amount = self.total_amount - tx.amount;
            Ok(())
        } else {
            Err(ChargebackError::InvalidClientId)
        }
    }
}

impl From<Account> for RawAccount {
    fn from(account: Account) -> RawAccount {
        RawAccount {
            client: account.client_id,
            available: round(account.available_amount),
            held: round(account.held_amount),
            total: round(account.total_amount),
            locked: account.is_locked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TransactionType;

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
        let transaction = Transaction::new(TransactionType::Deposit, client_id, 1, 25.0);
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
        let transaction = Transaction::new(TransactionType::Deposit, 2, 1, 25.0);

        let res = account.deposit(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, DepositError::InvalidClientId));
    }

    #[test]
    fn test_withdraw() {
        let client_id = 1;
        let mut account = Account::new(client_id);
        let deposit_transaction = Transaction::new(TransactionType::Deposit, client_id, 1, 25.0);
        let withdrawal_transaction =
            Transaction::new(TransactionType::Withdrawal, client_id, 1, 15.0);

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
        let transaction = Transaction::new(TransactionType::Withdrawal, client_id, 1, 25.0);

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(err, WithdrawalError::InsufficientFunds(client_id));
    }

    #[test]
    fn test_withdraw_invalid_client_id() {
        let mut account = Account::new(1);
        let transaction = Transaction::new(TransactionType::Withdrawal, 2, 1, 15.0);

        let res = account.withdraw(&transaction);
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(matches!(err, WithdrawalError::InvalidClientId));
    }
}

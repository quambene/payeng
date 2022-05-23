use crate::{
    errors::{DepositError, WithdrawalError},
    models::{
        round, RawAccount, {Transaction, TransactionType},
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

    pub fn deposit(&mut self, tx: &Transaction) -> Result<(), DepositError> {
        match tx.transaction_type {
            TransactionType::Deposit => {
                if self.client_id == tx.client_id {
                    if !self.is_locked {
                        self.available_amount += tx.amount;
                        self.total_amount += tx.amount;
                        Ok(())
                    } else {
                        Err(DepositError::FrozenAccount(self.client_id))
                    }
                } else {
                    Err(DepositError::InvalidClientId)
                }
            }
            _ => Err(DepositError::InvalidTransactionType(tx.transaction_id)),
        }
    }

    pub fn withdraw(&mut self, tx: &Transaction) -> Result<(), WithdrawalError> {
        match tx.transaction_type {
            TransactionType::Withdrawal => {
                if self.client_id == tx.client_id {
                    if !self.is_locked {
                        if self.available_amount - tx.amount >= 0.0 {
                            self.available_amount -= tx.amount;
                            self.total_amount -= tx.amount;
                            Ok(())
                        } else {
                            Err(WithdrawalError::InsufficientFunds(self.client_id))
                        }
                    } else {
                        Err(WithdrawalError::FrozenAccount(self.client_id))
                    }
                } else {
                    Err(WithdrawalError::InvalidClientId)
                }
            }
            _ => Err(WithdrawalError::InvalidTransactionType(tx.transaction_id)),
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
            Transaction::new(TransactionType::Withdrawal, client_id, 2, 15.0);

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

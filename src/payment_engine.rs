use crate::{
    errors::FormatError,
    models::{Account, Transaction, TransactionType},
};
use std::collections::{hash_map::Entry, HashMap};

pub fn process(
    transaction_history: &[u32],
    transactions: &HashMap<u32, Transaction>,
) -> Result<HashMap<u16, Account>, anyhow::Error> {
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    // Process transactions in chronological order
    for tx_id in transaction_history {
        match transactions.get(tx_id) {
            Some(transaction) => match transaction.transaction_type {
                TransactionType::Deposit => {
                    match accounts.entry(transaction.client_id) {
                        Entry::Occupied(entry) => {
                            let account = entry.into_mut();
                            account.deposit(transaction)?
                        }
                        Entry::Vacant(entry) => {
                            let account = entry.insert(Account::new(transaction.client_id));
                            account.deposit(transaction)?
                        }
                    }
                    // TODO: process transaction events for deposit
                }
                TransactionType::Withdrawal => {
                    match accounts.entry(transaction.client_id) {
                        Entry::Occupied(entry) => {
                            let account = entry.into_mut();
                            account.withdraw(transaction)?
                        }
                        Entry::Vacant(entry) => {
                            let account = entry.insert(Account::new(transaction.client_id));
                            account.withdraw(transaction)?
                        }
                    }
                    // TODO: process transaction events for withdrawal
                }
            },
            None => Err(FormatError::UniqueTransactionId(*tx_id))?,
        };
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let transaction_1 = Transaction::new(TransactionType::Deposit, 1, 1, 1.0);
        let transaction_2 = Transaction::new(TransactionType::Deposit, 2, 2, 2.0);
        let transaction_3 = Transaction::new(TransactionType::Deposit, 1, 3, 2.0);
        let transaction_4 = Transaction::new(TransactionType::Withdrawal, 1, 4, 1.5);
        let transaction_5 = Transaction::new(TransactionType::Withdrawal, 2, 5, 2.0);

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        transactions.insert(1, transaction_1);
        transactions.insert(2, transaction_2);
        transactions.insert(3, transaction_3);
        transactions.insert(4, transaction_4);
        transactions.insert(5, transaction_5);

        let transaction_history = [1, 2, 3, 4, 5];

        let res = process(&transaction_history, &transactions);
        assert!(res.is_ok());

        let accounts = res.unwrap();
        assert!(accounts.get(&1).is_some());
        assert!(accounts.get(&2).is_some());

        assert_eq!(
            accounts.get(&1).unwrap(),
            &Account {
                client_id: 1,
                available_amount: 1.5,
                held_amount: 0.0,
                total_amount: 1.5,
                is_locked: false
            }
        );
        assert_eq!(
            accounts.get(&2).unwrap(),
            &Account {
                client_id: 2,
                available_amount: 0.0,
                held_amount: 0.0,
                total_amount: 0.0,
                is_locked: false
            }
        );
    }
}

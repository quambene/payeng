use crate::{
    account::Account,
    transaction::{Transaction, TransactionType},
};
use std::collections::{hash_map::Entry, HashMap};

pub fn process(transactions: &[Transaction]) -> Result<HashMap<u16, Account>, anyhow::Error> {
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    // Process transactions in chronological order
    for tx in transactions {
        match tx.tx_type {
            TransactionType::Deposit => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.deposit(tx)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.deposit(tx)?
                }
            },
            TransactionType::Withdrawal => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.withdraw(tx)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.withdraw(tx)?
                }
            },
            TransactionType::Dispute => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.dispute(tx, &transactions)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.dispute(tx, &transactions)?
                }
            },
            TransactionType::Resolve => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.resolve(tx, &transactions)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.resolve(tx, &transactions)?
                }
            },
            TransactionType::Chargeback => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.chargeback(tx, &transactions)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.chargeback(tx, &transactions)?
                }
            },
        };
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let transactions = vec![
            Transaction::new(TransactionType::Deposit, 1, 1, Some(1.0)),
            Transaction::new(TransactionType::Deposit, 2, 2, Some(2.0)),
            Transaction::new(TransactionType::Deposit, 1, 3, Some(2.0)),
            Transaction::new(TransactionType::Withdrawal, 1, 4, Some(1.5)),
            Transaction::new(TransactionType::Withdrawal, 2, 5, Some(2.0)),
        ];

        let res = process(&transactions);
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

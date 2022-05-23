mod postprocessing;
mod preprocessing;

pub use postprocessing::postprocess;
pub use preprocessing::preprocess;

use crate::{
    errors::FormatError,
    models::{Account, EventType, Transaction, TransactionStatus, TransactionType},
};
use std::collections::{hash_map::Entry, HashMap};

pub fn process_transactions(
    transaction_history: &[u32],
    transactions: &mut HashMap<u32, Transaction>,
) -> Result<HashMap<u16, Account>, anyhow::Error> {
    // Use hash map for storing accounts; search, insertion and update is O(1)
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    // Process transactions in chronological order
    for id in transaction_history {
        match transactions.get_mut(id) {
            Some(tx) => match tx.transaction_type {
                TransactionType::Deposit => match accounts.entry(tx.client_id) {
                    Entry::Occupied(entry) => {
                        let account = entry.into_mut();
                        account.deposit(tx)?;
                        tx.status = TransactionStatus::Processed;
                        process_events(tx, account)?;
                    }
                    Entry::Vacant(entry) => {
                        let account = entry.insert(Account::new(tx.client_id));
                        account.deposit(tx)?;
                        tx.status = TransactionStatus::Processed;
                        process_events(tx, account)?;
                    }
                },
                TransactionType::Withdrawal => match accounts.entry(tx.client_id) {
                    Entry::Occupied(entry) => {
                        let account = entry.into_mut();
                        account.withdraw(tx)?;
                        tx.status = TransactionStatus::Processed;
                        process_events(tx, account)?;
                    }
                    Entry::Vacant(entry) => {
                        let account = entry.insert(Account::new(tx.client_id));
                        account.withdraw(tx)?;
                        tx.status = TransactionStatus::Processed;
                        process_events(tx, account)?;
                    }
                },
            },
            None => return Err(FormatError::UniqueTransactionId(*id).into()),
        };
    }

    Ok(accounts)
}

fn process_events(tx: &mut Transaction, account: &mut Account) -> Result<(), anyhow::Error> {
    if !tx.events.is_empty() {
        for event in tx.events.iter() {
            match event {
                EventType::Dispute => {
                    account.dispute(tx, event)?;
                    tx.status = TransactionStatus::Disputed;
                }
                EventType::Resolve => {
                    // Ignore resolve if transaction isn't under dispute
                    if tx.status == TransactionStatus::Disputed {
                        account.resolve(tx, event)?;
                        tx.status = TransactionStatus::Resolved;
                    }
                }
                EventType::Chargeback => {
                    // Ignore chargeback if transaction isn't under dispute
                    if tx.status == TransactionStatus::Disputed {
                        account.chargeback(tx, event)?;
                        tx.status = TransactionStatus::Reversed;
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::RawTransaction, payment_engine};

    #[test]
    fn test_process_transactions() {
        let raw_transactions: Vec<RawTransaction> = vec![
            RawTransaction::new(String::from("deposit"), 1, 1, Some(1.0)),
            RawTransaction::new(String::from("deposit"), 2, 2, Some(2.0)),
            RawTransaction::new(String::from("deposit"), 1, 3, Some(2.0)),
            RawTransaction::new(String::from("withdrawal"), 1, 4, Some(1.5)),
            RawTransaction::new(String::from("withdrawal"), 2, 5, Some(2.0)),
        ];
        let (transaction_history, mut transactions) =
            payment_engine::preprocess(raw_transactions).unwrap();

        let res = process_transactions(&transaction_history, &mut transactions);
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

    #[test]
    fn test_process_transactions_and_events() {
        let raw_transactions: Vec<RawTransaction> = vec![
            RawTransaction::new(String::from("deposit"), 1, 1, Some(1.0)),
            RawTransaction::new(String::from("deposit"), 2, 2, Some(2.0)),
            RawTransaction::new(String::from("deposit"), 1, 3, Some(2.0)),
            RawTransaction::new(String::from("withdrawal"), 1, 4, Some(1.5)),
            RawTransaction::new(String::from("dispute"), 1, 4, None),
            RawTransaction::new(String::from("chargeback"), 1, 4, None),
            RawTransaction::new(String::from("withdrawal"), 2, 5, Some(2.0)),
        ];
        let (transaction_history, mut transactions) =
            payment_engine::preprocess(raw_transactions).unwrap();

        let res = process_transactions(&transaction_history, &mut transactions);
        assert!(res.is_ok());

        let accounts = res.unwrap();
        assert!(accounts.get(&1).is_some());
        assert!(accounts.get(&2).is_some());

        assert_eq!(
            accounts.get(&1).unwrap(),
            &Account {
                client_id: 1,
                available_amount: 3.0,
                held_amount: 0.0,
                total_amount: 3.0,
                is_locked: true
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

    #[test]
    fn test_process_dispute_event() {
        let mut account = Account::new(1);

        let mut transaction = Transaction::new(TransactionType::Deposit, 1, 1, 1.0);
        transaction.events = vec![EventType::Dispute];

        let res = process_events(&mut transaction, &mut account);
        assert!(res.is_ok());

        assert_eq!(transaction.status, TransactionStatus::Disputed);
    }

    #[test]
    fn test_process_resolve_event() {
        let mut account = Account::new(1);

        let mut transaction = Transaction::new(TransactionType::Deposit, 1, 1, 1.0);
        transaction.events = vec![EventType::Dispute, EventType::Resolve];

        let res = process_events(&mut transaction, &mut account);
        assert!(res.is_ok());

        assert_eq!(transaction.status, TransactionStatus::Resolved);
    }

    #[test]
    fn test_process_chargeback_event() {
        let mut account = Account::new(1);

        let mut transaction = Transaction::new(TransactionType::Deposit, 1, 1, 1.0);
        transaction.events = vec![EventType::Dispute, EventType::Chargeback];

        let res = process_events(&mut transaction, &mut account);
        assert!(res.is_ok());

        assert_eq!(transaction.status, TransactionStatus::Reversed);
    }
}

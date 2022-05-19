use crate::{account::Account, transaction::Transaction};
use std::collections::{hash_map::Entry, HashMap};

pub fn process(transactions: &[Transaction]) -> Result<HashMap<u16, Account>, anyhow::Error> {
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    // Process transactions in chronological order
    for transaction in transactions {
        match transaction {
            Transaction::Deposit(tx) => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.deposit(tx)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.deposit(tx)?
                }
            },
            Transaction::Withdrawal(tx) => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();
                    account.withdraw(tx)?
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));
                    account.withdraw(tx)?
                }
            },
            // TODO: implement dispute, resolve, and chargeback transaction types
            _ => todo!(),
        };
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use crate::transaction::{DepositTransaction, WithdrawalTransaction};

    use super::*;

    #[test]
    fn test_process() {
        let transactions = vec![
            Transaction::Deposit(DepositTransaction::new(1, 1, 1.0)),
            Transaction::Deposit(DepositTransaction::new(2, 2, 2.0)),
            Transaction::Deposit(DepositTransaction::new(1, 3, 2.0)),
            Transaction::Withdrawal(WithdrawalTransaction::new(1, 4, 1.5)),
            Transaction::Withdrawal(WithdrawalTransaction::new(2, 5, 2.0)),
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

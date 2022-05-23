use std::collections::{hash_map::Entry, HashMap};

use crate::{
    errors::FormatError,
    models::{CheckedTransaction, RawTransaction, Transaction},
};

pub fn preprocess(
    raw_transactions: Vec<RawTransaction>,
) -> Result<(Vec<u32>, HashMap<u32, Transaction>), anyhow::Error> {
    // Collect time-ordered transaction ids in transaction_history; transactions have to be processed in chronological order
    let mut transaction_history: Vec<u32> = vec![];

    // The transaction events (dispute, resolve, chargeback) are aggregated into the transactions so that transaction_id is unique in the input data. This way transactions can be stored in a hash map. Otherwise, search in array would be O(n).
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    for raw_transaction in raw_transactions {
        // Check and verify input format via CheckedTransaction type
        let checked_transaction: CheckedTransaction = raw_transaction.try_into()?;

        match checked_transaction {
            CheckedTransaction::Transaction(tx) => match transactions.entry(tx.transaction_id) {
                Entry::Occupied(_) => {
                    return Err(FormatError::UniqueTransactionId(tx.transaction_id).into());
                }
                Entry::Vacant(entry) => {
                    transaction_history.push(tx.transaction_id);
                    entry.insert(tx);
                }
            },
            CheckedTransaction::TransactionEvent(event) => {
                match transactions.get_mut(&event.transaction_id) {
                    Some(transaction) => {
                        if transaction.client_id == event.client_id {
                            // Events are aggregated in chronological order
                            transaction.events.push(event.event_type)
                        } else {
                            // Assumption: client_id and transaction_id of the transaction event have to coincide with the actual transaction; ignore if this is not the case
                            continue;
                        }
                    }
                    // Assumption: transaction events which do not reference a valid transaction_id can be ignored
                    None => continue,
                }
            }
        };
    }

    Ok((transaction_history, transactions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EventType, TransactionType};

    #[test]
    fn test_preprocess() {
        let raw_transactions: Vec<RawTransaction> = vec![
            RawTransaction::new(String::from("deposit"), 1, 1, Some(1.0)),
            RawTransaction::new(String::from("deposit"), 2, 2, Some(2.0)),
            RawTransaction::new(String::from("deposit"), 1, 3, Some(2.0)),
            RawTransaction::new(String::from("withdrawal"), 1, 4, Some(1.5)),
            RawTransaction::new(String::from("dispute"), 1, 4, None),
            RawTransaction::new(String::from("chargeback"), 1, 4, None),
            RawTransaction::new(String::from("withdrawal"), 2, 5, Some(2.0)),
        ];

        let res = preprocess(raw_transactions);
        assert!(res.is_ok());

        let (transaction_history, transactions) = res.unwrap();
        assert_eq!(transaction_history, vec![1, 2, 3, 4, 5]);

        let target = vec![
            Transaction::new(TransactionType::Deposit, 1, 1, 1.0),
            Transaction::new(TransactionType::Deposit, 2, 2, 2.0),
            Transaction::new(TransactionType::Deposit, 1, 3, 2.0),
            Transaction::with_events(
                TransactionType::Withdrawal,
                1,
                4,
                1.5,
                vec![EventType::Dispute, EventType::Chargeback],
            ),
            Transaction::new(TransactionType::Withdrawal, 2, 5, 2.0),
        ];

        for transaction_id in transaction_history {
            assert_eq!(
                transactions.get(&transaction_id).unwrap(),
                target
                    .iter()
                    .find(|el| el.transaction_id == transaction_id)
                    .unwrap()
            );
        }
    }
}

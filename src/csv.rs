use crate::{
    errors::FormatError,
    models::{Account, CheckedTransaction, RawAccount, RawTransaction, Transaction},
};
use anyhow::Context;
use std::collections::{hash_map::Entry, HashMap};

pub fn read(csv_file: &str) -> Result<(Vec<u32>, HashMap<u32, Transaction>), anyhow::Error> {
    // Prepare csv reader and remove/ignore all whitespaces
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(csv_file)
        .context(format!("Can't find csv file at path '{}'", csv_file))?;

    // Collect time-ordered transaction ids in transaction_history; transactions have to be processed in chronological order
    let mut transaction_history: Vec<u32> = vec![];

    // The transaction events (dispute, resolve, chargeback) are aggregated into the transactions so that transaction_id is unique in the input data
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();

    // Prepare all transactions for processing (read from file, conversion to business objects)
    for record in csv_reader.deserialize() {
        let raw_transaction: RawTransaction = record?;

        // Check and verify input format via type CheckedTransaction
        let checked_transaction: CheckedTransaction = raw_transaction.try_into()?;

        match checked_transaction {
            CheckedTransaction::Transaction(tx) => match transactions.entry(tx.transaction_id) {
                Entry::Occupied(_) => {
                    return Err(FormatError::UniqueTransactionId(tx.transaction_id))?;
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

    return Ok((transaction_history, transactions));
}

pub fn write(accounts: HashMap<u16, Account>) -> Result<(), anyhow::Error> {
    // Prepare csv writer and configure to write csv records to stdout
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

    // Convert all business objects (Account) to the output format (RawAccount) and write to stdout in csv format
    for (_client_id, account) in accounts {
        let raw_account: RawAccount = account.into();
        csv_writer.serialize(raw_account)?;
    }

    Ok(())
}

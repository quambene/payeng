mod errors;
mod models;
mod payment_engine;

use crate::{
    errors::FormatError,
    models::{CheckedTransaction, RawAccount, RawTransaction, Transaction},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    env, io,
};

// TODO: Write test for different input files (with and without spaces)

// Output is parsed to stdout
// Errors are parsed to stderr via anyhow
fn main() -> anyhow::Result<()> {
    // Parse the command line arguments; the first argument (index 1) is the path to the input csv file
    let args: Vec<String> = env::args().collect();
    let csv_file = &args[1];

    // Prepare csv reader and remove/ignore all whitespaces
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(csv_file)?;

    // Prepare csv writer and configure to write csv records to stdout
    let mut csv_writer = csv::Writer::from_writer(io::stdout());

    // Collect time-ordered transaction ids in transaction_history; transactions have to be processed in chronological order
    let mut transaction_history: Vec<u32> = vec![];

    // The transaction events (dispute, resolved, chargeback) are aggregated into the transactions so that transaction_id is unique in the input data
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
                            transaction.events.push(event.event_type)
                        } else {
                            // Assumption: client_id and transaction_id of the transaction event have to coincide with the actual transaction; ignore if this is not the case
                            continue;
                        }
                    }
                    // Assumption: transaction events which do not reference a transaction id can be ignored
                    None => continue,
                }
            }
        };
    }

    // Process all transactions
    let accounts = payment_engine::process(&transaction_history, &transactions)?;

    // Convert all business objects (Account) to the output format (RawAccount) and write to stdout in csv format
    for (_client_id, account) in accounts {
        let raw_account: RawAccount = account.into();
        csv_writer.serialize(raw_account)?
    }

    Ok(())
}

mod account;
mod payment_engine;
mod raw_account;
mod transaction;

use crate::raw_account::RawAccount;
use crate::transaction::RawTransaction;
use std::{env, io};
use transaction::Transaction;

// TODO: Write test for different input files (with and without spaces)

// Errors are parsed to stderr (instead of stdout) via anyhow
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

    // Try to parse the complete CSV file at first; if an error occurs don't start processing and abort instead
    for record in csv_reader.deserialize() {
        let raw_transaction: RawTransaction = record?;
        let _transaction: Transaction = raw_transaction.try_into()?;
    }

    let mut transactions: Vec<Transaction> = vec![];

    // Process all transactions (if complete CSV file was parsed correctly)
    for res in csv_reader.deserialize() {
        let raw_transaction: RawTransaction = res?;
        let transaction: Transaction = raw_transaction.try_into()?;
        transactions.push(transaction)
    }

    let accounts = payment_engine::process(&transactions)?;

    for (_client_id, account) in accounts {
        let raw_account: RawAccount = account.into();
        csv_writer.serialize(raw_account)?
    }

    Ok(())
}

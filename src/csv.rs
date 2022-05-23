use crate::models::{RawAccount, RawTransaction};
use anyhow::Context;

pub fn read(csv_file: &str) -> Result<Vec<RawTransaction>, anyhow::Error> {
    // Prepare csv reader and remove/ignore all whitespaces
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(csv_file)
        .context(format!("Can't find csv file at path '{}'", csv_file))?;

    let mut raw_transactions = vec![];

    // Read from file an deserialize to RawTransaction type
    for record in csv_reader.deserialize() {
        let raw_transaction: RawTransaction = record?;
        raw_transactions.push(raw_transaction);
    }

    Ok(raw_transactions)
}

pub fn write(raw_accounts: Vec<RawAccount>) -> Result<(), anyhow::Error> {
    // Prepare csv writer and configure to write csv records to stdout
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

    // Serialize and write raw accounts to stdout
    for raw_account in raw_accounts {
        csv_writer.serialize(raw_account)?;
    }

    Ok(())
}

mod errors;
mod models;

use anyhow::anyhow;
use models::{
    Account, CheckedTransaction, RawTransaction, TransactionType, RawAccount,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    fs::File,
};

/*
    Output is parsed to stdout
    Errors are parsed to stderr via anyhow
    If an error occurs processing is aborted; the output file will remain empty
*/

fn main() -> Result<(), anyhow::Error> {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();

    let csv_file = if args.len() > 1 {
        // Second argument (index 1) is the path to the input csv file
        &args[1]
    } else {
        return Err(anyhow!(
            "Missing input file: please specify the path as argument"
        ));
    };

    chunkwise(csv_file)
}

// Demonstration of buffered reading (memory consumption keeps low)
fn chunkwise(csv_file: &str) -> Result<(), anyhow::Error> {
    let file = File::open(csv_file)?;

    // CSV reader is already buffered; no need for std::io::BufReader
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut accounts: HashMap<u16, Account> = HashMap::new();

    for record in csv_reader.deserialize() {
        let raw_transaction: RawTransaction = record?;
        let checked_transaction: CheckedTransaction = raw_transaction.try_into()?;

        match checked_transaction {
            CheckedTransaction::Transaction(tx) => match accounts.entry(tx.client_id) {
                Entry::Occupied(entry) => {
                    let account = entry.into_mut();

                    match tx.transaction_type {
                        TransactionType::Deposit => account.deposit(&tx)?,
                        TransactionType::Withdrawal => account.withdraw(&tx)?,
                    };
                }
                Entry::Vacant(entry) => {
                    let account = entry.insert(Account::new(tx.client_id));

                    match tx.transaction_type {
                        TransactionType::Deposit => account.deposit(&tx)?,
                        TransactionType::Withdrawal => account.withdraw(&tx)?,
                    };
                }
            },
            CheckedTransaction::TransactionEvent(_event) => {
                // One possible strategy for handling transaction events is described in the readme
                todo!()
            }
        }
    }

    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

    let raw_accounts = postprocess(accounts);

    for raw_account in raw_accounts {
        csv_writer.serialize(raw_account)?;
    }

    csv_writer.flush()?;

    Ok(())
}

fn postprocess(accounts: HashMap<u16, Account>) -> Result<Vec<RawAccount>, anyhow::Error> {
    let mut raw_accounts: Vec<RawAccount> = vec![];

    for (_client_id, account) in accounts {
        let raw_account = account.into();
        raw_accounts.push(raw_account);
    }

    Ok(raw_accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RawTransaction;
    use ::csv::Writer;
    use std::fs;

    #[test]
    #[ignore = "performance test"]
    fn test_performance() {
        fs::create_dir_all("tmp").unwrap();
        let csv_path = "tmp/transactions.csv";
        let mut csv_writer = Writer::from_path(csv_path).unwrap();

        /*
            max_value of 1_000_000 corresponds roughly to 20 MB of file size
            max_value of 10_000_000 corresponds roughly to 200 MB of file size
            max_value of 100_000_000 corresponds roughly to 2 GB of file size
        */
        let max_value = 100_000_000;
        for i in 1..max_value {
            let raw_transaction = RawTransaction::new(String::from("deposit"), 1, i, Some(1.0));
            csv_writer.serialize(raw_transaction).unwrap();
        }
        csv_writer.flush().unwrap();

        let instant = std::time::Instant::now();
        let res = chunkwise(csv_path);
        let elapsed_time = instant.elapsed().as_millis();

        assert!(res.is_ok());

        println!("response time: {:?} ms", elapsed_time);

        fs::remove_file(csv_path).unwrap();
    }
}

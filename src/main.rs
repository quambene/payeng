mod csv;
mod errors;
mod models;
mod payment_engine;

use anyhow::anyhow;
use std::env;

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

    wrapper(csv_file)
}

// Thin wrapper for testing
fn wrapper(csv_file: &str) -> Result<(), anyhow::Error> {
    // Read raw transactions from csv file
    let raw_transactions = csv::read(csv_file)?;

    // Prepare transactions for processing and convert raw transactions to business objects
    let (transaction_history, mut transactions) = payment_engine::preprocess(raw_transactions)?;

    // Process all transactions
    let accounts = payment_engine::process_transactions(&transaction_history, &mut transactions)?;

    // Convert business objects from Account to RawAccount
    let raw_accounts = payment_engine::postprocess(accounts)?;

    // Write raw accounts to stdout in csv format
    csv::write(raw_accounts)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RawTransaction;
    use ::csv::Writer;
    use std::fs;

    #[test]
    fn test_wrapper() {
        let res = wrapper("test_data/transactions.csv");
        assert!(res.is_ok());
    }

    #[test]
    fn test_wrapper_whitespaces() {
        let res = wrapper("test_data/transactions_whitespaces.csv");
        assert!(res.is_ok());
    }

    #[test]
    fn test_wrapper_with_events() {
        let res = wrapper("test_data/transactions_with_events.csv");
        assert!(res.is_ok());
    }

    #[test]
    fn test_wrapper_invalid_transaction_type() {
        let res = wrapper("test_data/transactions_invalid_transaction_type.csv");
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Unexpected format: invalid transaction type 'unknown' in transaction id 5"
        );
    }

    #[test]
    fn test_wrapper_invalid_transaction_id() {
        let res = wrapper("test_data/transactions_invalid_transaction_id.csv");
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Unexpected format: transaction id 1 is not unique"
        );
    }

    #[test]
    fn test_wrapper_invalid_amount() {
        let res = wrapper("test_data/transactions_invalid_amount.csv");
        assert!(res.is_err());

        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Unexpected format: amount is negative, infinite or NaN for transaction id 1 and transaction type 'deposit'"
        );
    }

    #[test]
    fn test_wrapper_deserialize_error() {
        let res = wrapper("test_data/transactions_deserialize_error.csv");
        assert!(res.is_err());
    }

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
        let max_value = 1_000_000;
        for i in 1..max_value {
            let raw_transaction = RawTransaction::new(String::from("deposit"), 1, i, Some(1.0));
            csv_writer.serialize(raw_transaction).unwrap();
        }
        csv_writer.flush().unwrap();

        let instant = std::time::Instant::now();
        let res = wrapper(csv_path);
        let elapsed_time = instant.elapsed().as_millis();

        assert!(res.is_ok());

        println!("response time: {:?} ms", elapsed_time);
        assert!(elapsed_time < 50000);

        fs::remove_file(csv_path).unwrap();
    }
}

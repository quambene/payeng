mod csv;
mod errors;
mod models;
mod payment_engine;

use anyhow::anyhow;
use std::env;

// Output is parsed to stdout
// Errors are parsed to stderr via anyhow
fn main() -> Result<(), anyhow::Error> {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();

    let csv_file = if args.len() > 1 {
        // First argument (index 1) is the path to the input csv file
        &args[1]
    } else {
        return Err(anyhow!(
            "Missing input file: please specify the path as argument"
        ))?;
    };

    wrapper(csv_file)
}

// Thin wrapper for testing
fn wrapper(csv_file: &str) -> Result<(), anyhow::Error> {
    // Read from file and convert RawTransactions to business objects
    let (transaction_history, transactions) = csv::read(csv_file)?;

    // Process all transactions
    let accounts = payment_engine::process(&transaction_history, &transactions)?;

    // Convert business objects from Account to RawAccount and write to stdout in csv format
    csv::write(accounts)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_wrapper_invalid_type() {
        let res = wrapper("test_data/transactions.csv");
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
}

# Payeng

A simple payments engine for processing transactions

- [Install Payeng](#install-payeng)
- [Usage](#usage)
- [Testing](#testing)
- [Architecture](#architecture)
- [Correctness, completeness, and safety](#correctness-completeness-and-safety)
- [Performance](#performance)

## Install Payeng

``` bash
git clone git@github.com:quambene/payeng.git

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

## Usage

``` bash
# Run in debug mode
cargo run -- transactions.csv > accounts.csv

# Run in release mode
cargo run --release -- transactions.csv > accounts.csv
```

## Testing

``` bash
# Run unit and integration tests
cargo test

# Run performance test in release mode
cargo test --release test_performance -- --ignored
```

## Architecture

Processing of transactions is divided into 5 steps:

1. **Read from csv**: Read raw transactions from csv file
2. **Preprocessing**: Prepare transactions for processing and convert raw transactions to business objects
3. **Processing**: Process all transactions chronologically and book/aggregate transactions on the client accounts
4. **Postprocessing**: Convert business objects to raw accounts
5. **Write to csv**: Write raw accounts to stdout in csv format

The core steps 2 to 4 are handled in module `payment_engine`. Reading and writing csv files (step 1 and 5) are handled in module `csv`.

Raw transactions are subdivided into

- `Transaction`s (deposit, withdrawal), and
- `TransactionEvent`s (dispute, resolve, chargeback) which affect existing  `Transaction`s

The business object `Transaction` includes its time-ordered transaction events as attribute.

Furthermore, the `transaction_history` includes all transactions IDs in chronological order. To prevent expensive searching in the transaction history, all transactions are saved in a `HashMap`.

Client accounts are stored in the business object `Account`. Since client accounts are searched and updated often, these are stored in a `HashMap` as well.

## Correctness, completeness, and safety

Correctness and completeness is ensured by exhaustive unit testing. Test data are included in the tests itself or in the `test_data` directory. Run all tests as described above.

The input format and data type of the `transactions.csv` file is ensured via the helper type `CheckedTransaction`.

Errors are parsed to stderr via `anyhow`. If an error occurs processing is aborted; the output file will remain empty. Safety relevant errors are handled by typed errors via `thiserror`. Error scenarios are validated by `match`ing the relevant error type in unit testing.

## Performance

This solution focuses on correctness, completeness, safety, and maintainable code.

Performance gets problematic beginning at approximately a few million lines in the `transactions.csv` file as validated by performance testing.

For example, the file size is approximately 2GB for 100 million lines. This space complexity can be handled by chunk-wise processing of the input file. One strategy is to preload all transaction events (dispute, resolve, chargeback) in a lookup table as searching all chunks for the referenced transactions would otherwise be quite slow. The assumption here being that transaction events do not occur as often as deposit and withdrawal transactions.

The incomplete version with chunk-wise processing can be found on branch `performance`:

``` bash
git checkout performance

# Run performance test with 100 million transactions as input (~2GB file size)
cargo test --release test_performance -- --ignored
```

This version on branch `performance` demontrates the use of buffered reading. Notice that the memory footprint keeps low.

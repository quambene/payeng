use super::{Transaction, TransactionEvent};

// Helper type: this type is used to check the format of the input csv file
#[derive(Debug)]
pub enum CheckedTransaction {
    Transaction(Transaction),
    TransactionEvent(TransactionEvent),
}

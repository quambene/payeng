use super::EventType;

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

// The workflow of a Transaction is described by its TransactionStatus
#[derive(Debug, PartialEq)]
pub enum TransactionStatus {
    // Transaction prepared for processing
    Initiated,
    // Transaction processed sucessfully
    Processed,
    // Transaction is disputed
    Disputed,
    // Dispute is resolved
    Resolved,
    // Dispute is resolved by reversing the transaction (corresponding to a chargeback)
    Reversed,
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client_id: u16,
    pub transaction_id: u32,
    pub amount: f64,
    pub events: Vec<EventType>,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn new(
        transaction_type: TransactionType,
        client_id: u16,
        transaction_id: u32,
        amount: f64,
    ) -> Self {
        Self {
            transaction_type,
            client_id,
            transaction_id,
            amount,
            events: vec![],
            status: TransactionStatus::Initiated,
        }
    }
}

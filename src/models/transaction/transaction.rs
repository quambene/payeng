use super::EventType;

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

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
    // Dispute is resolve by reversing the transaction (corresponds to chargeback performed)
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

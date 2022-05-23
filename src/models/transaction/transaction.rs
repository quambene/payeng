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
    _Processed,
    // Transaction is disputed
    _Disputed,
    // Dispute is resolved
    _Resolved,
    // Dispute is resolved by reversing the transaction (corresponding to a chargeback)
    _Reversed,
}

#[derive(Debug, PartialEq)]
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

    // Used in tests
    #[allow(dead_code)]
    pub fn with_events(
        transaction_type: TransactionType,
        client_id: u16,
        transaction_id: u32,
        amount: f64,
        events: Vec<EventType>,
    ) -> Self {
        Self {
            transaction_type,
            client_id,
            transaction_id,
            amount,
            events,
            status: TransactionStatus::Initiated,
        }
    }
}

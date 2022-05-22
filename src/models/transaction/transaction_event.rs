#[derive(Debug)]
pub enum EventType {
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug)]
pub struct TransactionEvent {
    pub event_type: EventType,
    pub client_id: u16,
    pub transaction_id: u32,
}

impl TransactionEvent {
    pub fn new(event_type: EventType, client_id: u16, transaction_id: u32) -> Self {
        Self {
            event_type,
            client_id,
            transaction_id,
        }
    }
}

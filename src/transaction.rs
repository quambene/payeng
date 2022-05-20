#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub client_id: u16,
    pub transaction_id: u32,
    pub amount: Option<f64>,
}

impl Transaction {
    pub fn new(
        tx_type: TransactionType,
        client_id: u16,
        transaction_id: u32,
        amount: Option<f64>,
    ) -> Self {
        Self {
            tx_type,
            client_id,
            transaction_id,
            amount,
        }
    }
}

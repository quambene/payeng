/*
    TODO: performance considerations
    - check type alignments like #[repr(C)] for Transaction
*/

// TODO: check decimal precision
// TODO: implement new type pattern for client_id and transaction_id

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
}

pub struct Transaction {
    // TODO: improve naming for "tx_type": name does contain superfluous information, but "type" is a keyword
    pub tx_type: TransactionType,
    pub client_id: u16,
    pub transaction_id: u32,
    pub amount: f64,
}

impl Transaction {
    pub fn new(tx_type: TransactionType, client_id: u16, transaction_id: u32, amount: f64) -> Self {
        Self {
            tx_type,
            client_id,
            transaction_id,
            amount,
        }
    }
}

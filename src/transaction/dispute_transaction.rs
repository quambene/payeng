use super::TransferTransaction;

#[derive(Debug)]
pub struct DisputeTransaction {
    pub client_id: u16,
    pub transaction_id: u32,
    pub transaction: TransferTransaction,
}

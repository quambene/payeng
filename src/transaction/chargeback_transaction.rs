use super::DisputeTransaction;

#[derive(Debug)]
pub struct ChargebackTransaction {
    pub client_id: u16,
    pub transaction_id: u32,
    pub transaction: DisputeTransaction,
}
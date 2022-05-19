#[derive(Debug)]
pub struct WithdrawalTransaction {
    pub client_id: u16,
    pub transaction_id: u32,
    pub amount: f64,
}

impl WithdrawalTransaction {
    pub fn new(client_id: u16, transaction_id: u32, amount: f64) -> Self {
        Self {
            client_id,
            transaction_id,
            amount,
        }
    }
}

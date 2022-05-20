#[derive(Debug)]
pub struct ChargebackTransaction {
    pub client_id: u16,
    pub transaction_id: u32,
}

impl ChargebackTransaction {
    pub fn new(client_id: u16, transaction_id: u32) -> Self {
        Self {
            client_id,
            transaction_id,
        }
    }
}

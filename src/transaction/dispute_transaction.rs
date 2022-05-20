#[derive(Debug)]
pub struct DisputeTransaction {
    pub client_id: u16,
    pub transaction_id: u32,
}

impl DisputeTransaction {
    pub fn new(client_id: u16, transaction_id: u32) -> Self {
        Self {
            client_id,
            transaction_id,
        }
    }
}

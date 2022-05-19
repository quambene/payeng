
#[derive(Debug)]
pub struct RawTransaction {
    pub tx_type: String,
    pub client_id: u16,
    pub transaction_id: u32,
    pub amount: Option<f64>,
}

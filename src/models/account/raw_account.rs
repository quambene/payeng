use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct RawAccount {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl RawAccount {
    // Used in tests
    #[allow(dead_code)]
    pub fn new(client: u16, available: f64, held: f64, total: f64, locked: bool) -> Self {
        Self {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}

mod account;
mod transaction;

pub use account::{Account, RawAccount};
pub use transaction::{
    CheckedTransaction, EventType, RawTransaction, Transaction, TransactionEvent,
    TransactionStatus, TransactionType,
};

const PRECISION: f64 = 10000.;

pub fn round(amount: f64) -> f64 {
    (amount * PRECISION).round() / PRECISION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up() {
        assert_eq!(round(42.34578), 42.3458)
    }

    #[test]
    fn test_round_down() {
        assert_eq!(round(42.34574), 42.3457)
    }

    #[test]
    fn test_round_incorrect() {
        assert_ne!(round(42.34578), 42.3457)
    }
}

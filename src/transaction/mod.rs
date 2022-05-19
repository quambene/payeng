mod chargeback_transaction;
mod deposit_transaction;
mod dispute_transaction;
mod errors;
mod raw_transaction;
mod resolve_transaction;
mod withdrawal_transaction;

pub use chargeback_transaction::ChargebackTransaction;
pub use deposit_transaction::DepositTransaction;
pub use dispute_transaction::DisputeTransaction;
pub use errors::{ChargebackError, DepositError, DisputeError, ResolveError, WithdrawalError};
pub use raw_transaction::RawTransaction;
pub use resolve_transaction::ResolveTransaction;
pub use withdrawal_transaction::WithdrawalTransaction;

#[derive(Debug)]
pub enum Transaction {
    Deposit(DepositTransaction),
    Withdrawal(WithdrawalTransaction),
    Dispute(DisputeTransaction),
    Resolve(ResolveTransaction),
    Chargeback(ChargebackTransaction),
}

#[derive(Debug)]
pub enum TransferTransaction {
    Deposit(DepositTransaction),
    Withdrawal(WithdrawalTransaction),
}

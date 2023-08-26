use candid::{CandidType, Nat, Principal};
use serde_derive::Deserialize;

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

pub type DepositReceipt = Result<Nat, TxError>;
pub type WithdrawReceipt = Result<Nat, TxError>;

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
}

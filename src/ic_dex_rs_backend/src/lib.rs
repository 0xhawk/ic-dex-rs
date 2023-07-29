use std::cell::RefCell;

use candid::{CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub type DepositReceipt = Result<Nat, DepositErr>;

#[derive(CandidType, PartialEq, Debug)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
    // exchange: Exchange,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

pub fn deposit(token_canister_id: Principal) -> DepositReceipt {
    // let caller = caller();
    
    let amount = Nat::from(0);
    DepositReceipt::Ok(amount)
}

#[test]
fn test_deposit() {
    let token_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let amount = Nat::from(0);
    let receipt = deposit(token_canister_id);
    assert_eq!(receipt, DepositReceipt::Ok(amount));
}
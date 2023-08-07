use std::cell::RefCell;
use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;

mod exchange;
mod types;
mod utils;

use exchange::Exchange;
use types::*;
use utils::principal_to_subaccount;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
    exchange: Exchange,
}


use std::cell::RefCell;

use candid::{CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::*;
use utils::principal_to_subaccount;

// mod ft;
mod utils;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

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

#[init]
fn init(ledger: Option<Principal>) {
    ic_cdk::setup();
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
        s.borrow_mut().ledger = ledger;
    });
}

#[ic_cdk::query]
#[candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn owner() -> Principal {
    STATE.with(|s| s.borrow().owner.unwrap())
}

fn deposit_icp(caller: Principal) -> Nat {
    Nat::from(0)
}

fn deposit_token(caller: Principal, token: Principal) -> Nat {
    Nat::from(0)
}

#[update]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    let caller = caller();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
    let amount = if token_canister_id == ledger_canister_id {
        deposit_icp(caller)
    } else {
        deposit_token(caller, token_canister_id)
    };
    DepositReceipt::Ok(amount)
}

#[update]
#[candid_method]
pub fn clear() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        assert!(state.owner.unwrap() == caller());
    })
}

use ic_cdk::export::candid::export_service;

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        write(dir.join("ic_dex_rs_backend.did"), export_candid()).expect("Write failed.");
    }
}

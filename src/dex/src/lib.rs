use candid::{candid_method, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;

mod exchange;
mod types;
mod utils;

use exchange::Exchange;
use types::*;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
    exchange: Exchange,
}

#[update]
#[candid_method(update)]
pub async fn deposit(amount: Nat, token_canister_id: Principal) -> DepositReceipt {
    let caller = caller();
    let amount = deposit_token(caller, &amount, token_canister_id.clone()).await?;
    STATE.with(|s| {
        s.borrow_mut()
            .exchange
            .balances
            .add_balance(&caller, &token_canister_id, amount.to_owned())
    });
    let balance = get_balance(caller, token_canister_id);
    ic_cdk::println!("Deposited Balance: {}", balance);
    DepositReceipt::Ok(amount)
}

#[update]
#[candid_method(update)]
pub async fn withdraw(
    amount: Nat,
    token_canister_id: Principal,
    dest: Principal,
) -> WithdrawReceipt {
    withdraw_token(dest, &amount, token_canister_id).await
}

pub type TxReceipt = Result<Nat, TxError>;

async fn deposit_token(
    caller: Principal,
    amount: &Nat,
    token_canister_id: Principal,
) -> Result<Nat, TxError> {
    let self_principal = ic_cdk::api::id();

    let call_result: Result<(TxReceipt,), _> = ic_cdk::api::call::call(
        token_canister_id,
        "transfer_from",
        (caller, self_principal, amount),
    )
    .await;

    let call_result: Result<Nat, TxError> = call_result.unwrap().0;
    ic_cdk::println!("Deposit of {} ICP in account {:?}", amount, &caller);
    call_result
}

async fn withdraw_token(
    dest: Principal,
    amount: &Nat,
    token_canister_id: Principal,
) -> Result<Nat, TxError> {
    let caller = caller();
    let sufficient_balance = STATE.with(|s| {
        s.borrow_mut()
            .exchange
            .balances
            .subtract_balance(&caller, &token_canister_id, amount)
    });
    if !sufficient_balance {
        return Err(TxError::InsufficientBalance);
    }

    let call_result: Result<(TxReceipt,), _> =
        ic_cdk::api::call::call(token_canister_id, "transfer", (dest, amount)).await;

    let call_result: Result<Nat, TxError> = call_result.unwrap().0;
    ic_cdk::println!("Withdrawal of {} ICP to account {:?}", amount, &dest);
    call_result
}

#[init]
fn init(ledger: Option<Principal>) {
    ic_cdk::setup();
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
        s.borrow_mut().ledger = ledger;
    });
}

pub fn clear() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        assert!(state.owner == Some(caller()));
        state.exchange.balances.0.clear();
    })
}

#[query]
#[candid_method(query)]
pub fn whoami() -> Principal {
    caller()
}

#[query]
#[candid_method(query)]
pub fn get_balance(owner: Principal, token_canister_id: Principal) -> Nat {
    STATE.with(|s| s.borrow().exchange.get_balance(owner, token_canister_id))
}

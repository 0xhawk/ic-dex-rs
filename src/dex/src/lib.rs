use candid::{candid_method, CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use std::cell::RefCell;

mod icrc2;

use icrc2::{new_transfer_from, ICRC2};

pub type DepositReceipt = Result<Nat, DepositErr>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
}

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

#[update]
#[candid_method(update)]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    let caller = caller();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let amount = if token_canister_id == ledger_canister_id {
        deposit_icp(caller).await?
    } else {
        deposit_icrc2(caller, token_canister_id).await?
    };
    // TODO
    DepositReceipt::Ok(amount)
}

async fn deposit_icrc2(caller: Principal, token: Principal) -> Result<Nat, DepositErr> {
    let token = ICRC2::new(token);

    let available = Nat::from(0);

    let transfer_from_args = new_transfer_from(
        caller,
        ic_cdk::api::id(),
        Nat::from(0),
        Option::None,
        Option::None,
        Option::None,
    );

    token
        .icrc2_transfer_from(transfer_from_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?;
    Ok(available)
}

async fn deposit_icp(caller: Principal) -> Result<Nat, DepositErr> {
    Ok((Nat::from(0)))
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

        // TODO
    })
}

use std::cell::RefCell;
use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;

mod dip20;
mod exchange;
mod types;
mod utils;

use dip20::DIP20;
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

#[update]
#[candid_method(update)]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    let caller = caller();
    let amount = Nat::from(0);
    DepositReceipt::Ok(amount)
}

async fn deposit_token(caller: Principal, token: Principal) -> Result<Nat, DepositErr> {
    let token = DIP20::new(token);
    let dip_fee = token.get_metadata().await.fee;
    let allowance = token.allowance(caller, ic_cdk::api::id()).await;

    let available = allowance - dip_fee;
    token.transfer_from(caller, ic_cdk::api::id(), available.to_owned())
        .await
        .map_err(|_| DepositErr::TransferFailure)?;
    
    Ok(available)
}
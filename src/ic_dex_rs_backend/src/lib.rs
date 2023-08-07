use candid::types::principal::Principal;
use candid::{candid_method, export_service, Nat};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT};
use std::cell::RefCell;

mod dip20;
mod exchange;
mod types;
mod utils;

use dip20::DIP20;
use exchange::Exchange;
use types::*;
use utils::principal_to_subaccount;

const ICP_FEE: u64 = 10_000;
pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

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
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
    let amount = if token_canister_id == ledger_canister_id {
        deposit_icp(caller).await?
    } else {
        deposit_token(caller, token_canister_id).await?
    };
    DepositReceipt::Ok(amount)
}

async fn deposit_token(caller: Principal, token: Principal) -> Result<Nat, DepositErr> {
    let token = DIP20::new(token);
    let dip_fee = token.get_metadata().await.fee;
    let allowance = token.allowance(caller, ic_cdk::api::id()).await;

    let available = allowance - dip_fee;
    token
        .transfer_from(caller, ic_cdk::api::id(), available.to_owned())
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    Ok(available)
}

async fn deposit_icp(caller: Principal) -> Result<Nat, DepositErr> {
    let canister_id = ic_cdk::api::id();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));
    let balance_args = ic_ledger_types::AccountBalanceArgs { account };
    let balance = ic_ledger_types::account_balance(ledger_canister_id, balance_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    Ok((balance.e8s() - ICP_FEE).into())
}

use candid::{candid_method, CandidType, Int, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};
use serde_derive::Deserialize;
use std::cell::RefCell;

mod exchange;
mod stable;
mod types;
mod utils;

use exchange::Exchange;
use types::*;
use utils::principal_to_subaccount;

pub type DepositReceipt = Result<Nat, TxError>;
pub type WithdrawReceipt = Result<Nat, WithdrawErr>;

const ICP_FEE: u64 = 10_000;

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
    DepositReceipt::Ok(amount)
}

#[update]
#[candid_method(update)]
pub async fn withdraw(amount: Nat, address: Principal) -> WithdrawReceipt {
    let caller = caller();

    STATE.with(|s| {
        s.borrow_mut()
            .exchange
            .orders
            .retain(|_, v| v.owner != caller);
    });
    let account_id = AccountIdentifier::new(&address, &DEFAULT_SUBACCOUNT);
    withdraw_icp(&amount, account_id).await
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
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

    let call_result = call_result.unwrap().0;
    ic_cdk::println!("Deposit of {} ICP in account {:?}", amount, &caller);
    call_result
}

async fn withdraw_icp(amount: &Nat, account_id: AccountIdentifier) -> Result<Nat, WithdrawErr> {
    let caller = caller();
    let ledger_canister_id = ledger_canister_id();
    let sufficient_balance = STATE.with(|s| {
        s.borrow_mut().exchange.balances.subtract_balance(
            &caller,
            &ledger_canister_id,
            amount.to_owned() + ICP_FEE,
        )
    });
    if !sufficient_balance {
        return Err(WithdrawErr::BalanceLow);
    }
    let transfer_amount = Tokens::from_e8s(
        (amount.to_owned() + ICP_FEE)
            .0
            .try_into()
            .map_err(|_| WithdrawErr::TransferFailure)?,
    );
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: transfer_amount,
        fee: Tokens::from_e8s(ICP_FEE),
        from_subaccount: Some(DEFAULT_SUBACCOUNT),
        to: account_id,
        created_at_time: None,
    };
    let icp_receipt = ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)
        .and_then(|v| v.map_err(|_| WithdrawErr::TransferFailure));

    if let Err(e) = icp_receipt {
        STATE.with(|s| {
            s.borrow_mut().exchange.balances.add_balance(
                &caller,
                &ledger_canister_id,
                amount.to_owned() + ICP_FEE,
            )
        });
        return Err(e);
    }

    ic_cdk::println!("Withdrawal of {} ICP to account {:?}", amount, &account_id);

    Ok(amount.to_owned() + ICP_FEE)
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
        state.exchange.orders.clear();
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
pub fn get_balance() -> Nat {
    STATE.with(|s| s.borrow().exchange.get_balance(ledger_canister_id()))
}

fn ledger_canister_id() -> Principal {
    STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID)
}

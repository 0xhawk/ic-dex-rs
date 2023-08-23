// This is a temporary implementation until ICRC-2 WASM becomes available;
// it will be replaced when ICRC-2 compatible Ledger becomes available.
// This implementation is not intended to be used in production.
use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
}

pub type TxReceipt = Result<Nat, TxError>;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Nat>> = RefCell::new(HashMap::default());
    static ALLOWS: RefCell<HashMap<Principal, HashMap<Principal, Nat>>> = RefCell::new(HashMap::default());
    static STATS: RefCell<StatsData> = RefCell::new(StatsData::default());
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct Metadata {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    fee: Nat,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct StatsData {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    fee: Nat,
}

#[derive(Debug, Clone)]
pub struct TxRecord {
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub fee: Nat,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct TokenInfo {
    metadata: Metadata,
}

impl Default for StatsData {
    fn default() -> Self {
        StatsData {
            name: "ICP".to_string(),
            symbol: "ICP".to_string(),
            decimals: 0u8,
            total_supply: Nat::from(100_000_000),
            fee: Nat::from(0),
        }
    }
}

#[init]
fn init() {
    let default_stats = StatsData::default();
    STATS.with(|s| {
        let mut stats = s.borrow_mut();
        stats.name = default_stats.name;
        stats.symbol = default_stats.symbol;
        stats.decimals = default_stats.decimals;
        stats.total_supply = default_stats.total_supply;
        stats.fee = default_stats.fee;
    });

    let caller = ic_cdk::caller();
    let total_supply = STATS.with(|s| s.borrow().total_supply.clone());
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        balances.insert(caller, total_supply);
    });
}

#[update]
#[candid_method(update)]
async fn transfer(to: Principal, amount: Nat) -> TxReceipt {
    let from = ic_cdk::caller();
    _transfer(from, to, amount.clone());
    Ok(amount)
}

#[update]
#[candid_method(update)]
async fn transfer_from(from: Principal, to: Principal, amount: Nat) -> TxReceipt {
    let caller = ic_cdk::caller();
    let from_allowance = allowance(from, caller);
    if from_allowance < amount.clone() {
        return Err(TxError::InsufficientAllowance);
    }
    let from_balance = balance_of(from);
    if from_balance < amount.clone() {
        return Err(TxError::InsufficientBalance);
    }
    _transfer(from, to, amount.clone());

    ALLOWS.with(|a| {
        let mut allowances = a.borrow_mut();
        match allowances.get(&from) {
            Some(inner) => {
                let result = inner.get(&caller).unwrap().clone();
                let mut temp = inner.clone();
                if result.clone() - amount.clone() != 0 {
                    temp.insert(caller, result.clone() - amount.clone());
                    allowances.insert(from, temp);
                } else {
                    temp.remove(&caller);
                    if temp.len() == 0 {
                        allowances.remove(&from);
                    } else {
                        allowances.insert(from, temp);
                    }
                }
            }
            None => {
                assert!(false);
            }
        }
    });
    Ok(amount)
}

#[update]
#[candid_method(update)]
fn approve(spender: Principal, amount: Nat) -> TxReceipt {
    let caller = ic_cdk::caller();
    ALLOWS.with(|a| {
        let mut allowances = a.borrow_mut();
        match allowances.get(&caller) {
            Some(inner) => {
                let mut temp = inner.clone();
                if amount.clone() != 0 {
                    temp.insert(spender, amount.clone());
                    allowances.insert(caller, temp);
                } else {
                    temp.remove(&spender);
                    if temp.len() == 0 {
                        allowances.remove(&caller);
                    } else {
                        allowances.insert(caller, temp);
                    }
                }
            }
            None => {
                if amount.clone() != 0 {
                    let mut inner = HashMap::new();
                    inner.insert(spender, amount.clone());
                    allowances.insert(caller, inner);
                }
            }
        }
    });
    Ok(amount)
}

#[query]
#[candid_method(query)]
fn name() -> String {
    STATS.with(|s| s.borrow().name.clone())
}

#[query]
#[candid_method(query)]
fn symbol() -> String {
    STATS.with(|s| s.borrow().symbol.clone())
}

#[query]
#[candid_method(query)]
fn total_supply() -> Nat {
    STATS.with(|s| s.borrow().total_supply.clone())
}

#[query]
#[candid_method(query)]
fn balance_of(id: Principal) -> Nat {
    BALANCES.with(|b| {
        let balances = b.borrow();
        match balances.get(&id) {
            Some(balance) => balance.clone(),
            None => Nat::from(0),
        }
    })
}

#[query]
#[candid_method(query)]
fn allowance(owner: Principal, spender: Principal) -> Nat {
    ALLOWS.with(|a| {
        let allowances = a.borrow();
        match allowances.get(&owner) {
            Some(inner) => match inner.get(&spender) {
                Some(value) => value.clone(),
                None => Nat::from(0),
            },
            None => Nat::from(0),
        }
    })
}

fn _transfer(from: Principal, to: Principal, amount: Nat) {
    let from_balance = balance_of(from);
    let from_balance_new = from_balance - amount.clone();

    if from_balance_new != 0 {
        _balance_ins(from, from_balance_new);
    } else {
        _balance_rem(from);
    }

    let to_balance = balance_of(to);
    let to_balance_new = to_balance + amount.clone();
    if to_balance_new != 0 {
        _balance_ins(to, to_balance_new);
    }
}

fn _balance_ins(from: Principal, value: Nat) {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        balances.insert(from, value);
    });
}

fn _balance_rem(from: Principal) {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        balances.remove(&from);
    });
}

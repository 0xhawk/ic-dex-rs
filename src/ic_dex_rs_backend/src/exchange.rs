use std::collections::HashMap;
use candid::{Nat, Principal};
use ic_cdk::caller;
use crate::types::*;
use crate::{utils, OrderId};

#[derive(Default)]
pub struct Balances(pub HashMap<Principal, HashMap<Principal, Nat>>);
type Orders = HashMap<OrderId, Order>;

#[derive(Default)]
pub struct Exchange {
    pub next_id: OrderId,
    pub balances: Balances,
    pub orders: Orders,
}

impl Exchange {
    pub fn get_balance(&self, token_canister_id: Principal) -> Nat {
        self.balances
            .0
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(utils::zero(), |v| v.clone())
    }
}
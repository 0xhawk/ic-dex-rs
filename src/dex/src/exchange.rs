use candid::{Nat, Principal};
use std::collections::HashMap;

use crate::types::*;
use crate::{utils, OrderId};

#[derive(Default)]
pub struct Balances(pub HashMap<Principal, HashMap<Principal, Nat>>); // owner -> token_canister_id -> amount
type Orders = HashMap<OrderId, Order>;

#[derive(Default)]
pub struct Exchange {
    pub next_id: OrderId,
    pub balances: Balances,
    pub orders: Orders,
}

impl Balances {
    pub fn add_balance(&mut self, owner: &Principal, token_canister_id: &Principal, delta: Nat) {
        let balances = self.0.entry(*owner).or_insert_with(HashMap::new);

        if let Some(x) = balances.get_mut(token_canister_id) {
            *x += delta;
        } else {
            balances.insert(*token_canister_id, delta);
        }
    }

    // Tries to substract balance from user account. Checks for overflows
    pub fn subtract_balance(
        &mut self,
        owner: &Principal,
        token_canister_id: &Principal,
        amount: &Nat,
    ) -> bool {
        let mut temp_map = HashMap::new();
        let mut zero = Nat::from(0);
        let b1 = self.0.get_mut(&owner).unwrap_or(&mut temp_map);
        let b2 = b1.get_mut(&token_canister_id).unwrap_or(&mut zero);

        let delta = amount.clone();
        if *b2 >= delta {
            *b2 -= delta;
            // no need to keep an empty token record
            if *b2 == utils::zero() {
                b1.remove(token_canister_id);
            }
            return true;
        } else {
            return false;
        }
    }
}

impl Exchange {
    pub fn get_balance(&self, owner: Principal, token_canister_id: Principal) -> Nat {
        self.balances
            .0
            .get(&owner)
            .and_then(|v| v.get(&token_canister_id))
            .map_or(utils::zero(), |v| v.to_owned())
    }
}

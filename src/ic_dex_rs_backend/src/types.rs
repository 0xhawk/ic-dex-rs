use candid::{CandidType, Nat, Principal};

pub type OrderId = u32;

#[allow(non_snake_case)]
#[derive(CandidType, Clone)]
pub struct Order {
    pub id: OrderId,
    pub owner: Principal,
    pub from: Principal,
    pub fromAmount: Nat,
    pub to: Principal,
    pub toAmount: Nat,
}

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

pub type CancelOrderReceipt = Result<OrderId, CancelOrderErr>;

#[derive(CandidType)]
pub enum CancelOrderErr {
    NotAllowed,
    NotExistingOrder,
}
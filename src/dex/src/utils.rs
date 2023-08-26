use candid::Nat;
use num_bigint::BigUint;
use num_traits::Zero;

pub fn zero() -> Nat {
    Nat(BigUint::zero())
}

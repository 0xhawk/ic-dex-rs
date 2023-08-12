#!/bin/bash
set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

export NETWORK=local

dfx identity use alice
export ALICE_PRINCIPAL=$(dfx identity get-principal)
dfx identity use bob
export BOB_PRINCIPAL=$(dfx identity get-principal)
dfx identity use minter
export MINTER_PRINCIPAL=$(dfx identity get-principal)

# === MINTER -> ALICE ====
dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_transfer "(record { 
  to = record { owner = principal \"${ALICE_PRINCIPAL}\";};
  amount = 20000000;
 })"

# === MINTER -> BOB ====
dfx canister --network ${NETWORK} call icrc1-bbb-ledger icrc1_transfer "(record { 
  to = record { owner = principal \"${BOB_PRINCIPAL}\";};
  amount = 30000000;
 })"

# === BALANCES ====
echo "A-Token: "
dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_balance_of "(record { owner = principal \"${MINTER_PRINCIPAL}\"; })"
dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_balance_of "(record { owner = principal \"${ALICE_PRINCIPAL}\"; })"
dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_balance_of "(record { owner = principal \"${BOB_PRINCIPAL}\"; })"
echo "B-Token: "
dfx canister --network ${NETWORK} call icrc1-bbb-ledger icrc1_balance_of "(record { owner = principal \"${MINTER_PRINCIPAL}\"; })"
dfx canister --network ${NETWORK} call icrc1-bbb-ledger icrc1_balance_of "(record { owner = principal \"${ALICE_PRINCIPAL}\"; })"
dfx canister --network ${NETWORK} call icrc1-bbb-ledger icrc1_balance_of "(record { owner = principal \"${BOB_PRINCIPAL}\"; })"

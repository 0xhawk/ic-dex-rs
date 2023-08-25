set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

# dfx identity new alice
dfx identity use alice
export ALICE_ACC=$(dfx ledger account-id)
export ALICE_PRINCIPAL=$(dfx identity get-principal)
echo $ALICE_ACC
echo $ALICE_PRINCIPAL

# dfx identity new bob
dfx identity use bob
export BOB_ACC=$(dfx ledger account-id)
export BOB_PRINCIPAL=$(dfx identity get-principal)
echo $BOB_ACC
echo $BOB_PRINCIPAL

# approve
export DEX_ID=$(dfx canister id dex)
dfx identity use alice
dfx canister call ledger approve "(principal \"$DEX_ID\", 1000)"
dfx identity use bob
dfx canister call ledger approve "(principal \"$DEX_ID\", 1000)"

# deposit
export LEDGER_ID=$(dfx canister id ledger)
dfx identity use alice
dfx canister call dex deposit "(200, principal \"$LEDGER_ID\")"
dfx identity use bob
dfx canister call dex deposit "(300, principal \"$LEDGER_ID\")"

echo "=== Alice Balance ==="
dfx canister call ledger balance_of "(principal \"$ALICE_PRINCIPAL\")"
echo "=== Bob Balance ==="
dfx canister call ledger balance_of "(principal \"$BOB_PRINCIPAL\")"
echo "=== DEX Balance ==="
dfx canister call ledger balance_of "(principal \"$DEX_ID\")"

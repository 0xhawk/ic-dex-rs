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

# transfer ICP tokens to users
dfx identity use default
dfx canister call ledger transfer "(principal \"$ALICE_PRINCIPAL\", 1000)"
dfx canister call ledger transfer "(principal \"$BOB_PRINCIPAL\", 1500)"

# balance check
echo "=== Alice Balance ==="
dfx canister call ledger balance_of "(principal \"$ALICE_PRINCIPAL\")"
echo "=== Bob Balance ==="
dfx canister call ledger balance_of "(principal \"$BOB_PRINCIPAL\")"

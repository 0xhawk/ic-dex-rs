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

echo "=== Alice Allowance ==="
dfx canister call ledger allowance "(principal \"$ALICE_PRINCIPAL\", principal \"$DEX_ID\")"
echo "=== Bob Allowance ==="
dfx canister call ledger allowance "(principal \"$BOB_PRINCIPAL\", principal \"$DEX_ID\")"

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

echo "=== Alice Allowance ==="
dfx canister call ledger allowance "(principal \"$ALICE_PRINCIPAL\", principal \"$DEX_ID\")"
echo "=== Bob Allowance ==="
dfx canister call ledger allowance "(principal \"$BOB_PRINCIPAL\", principal \"$DEX_ID\")"

# withdraw
dfx identity use alice
dfx canister call dex withdraw "(150, principal \"$LEDGER_ID\", principal \"$ALICE_PRINCIPAL\")"
dfx identity use bob
dfx canister call dex withdraw "(200, principal \"$LEDGER_ID\", principal \"$ALICE_PRINCIPAL\")"

echo "=== Alice Balance ==="
dfx canister call ledger balance_of "(principal \"$ALICE_PRINCIPAL\")"
echo "=== Bob Balance ==="
dfx canister call ledger balance_of "(principal \"$BOB_PRINCIPAL\")"
echo "=== DEX Balance ==="
dfx canister call ledger balance_of "(principal \"$DEX_ID\")"

set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

dfx stop && dfx start --background --clean

# dfx identity new minter
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

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

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)
export LEDGER_PRINCIPAL=$(dfx identity get-principal)

# deploy all canisters
dfx deploy ledger
dfx deploy system_api
dfx deploy vetkd_backend
dfx deploy front
export LEDGER_ID=$(dfx canister id ledger)
dfx deploy dex --argument "(opt principal \"$LEDGER_ID\")"

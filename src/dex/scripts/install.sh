set -e
dfx stop && dfx start --background --clean

### === DEPLOY LOCAL LEDGER =====
# dfx identity new minter
dfx identity use minter
export MINTER_ACC=$(dfx ledger account-id)
export MINTER_PRINCIPAL=$(dfx identity get-principal)
echo $MINTER_ACC
echo $MINTER_PRINCIPAL

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

# === DEPLOY TOKENS ====
sh src/icrc1/scripts/deploy_aaa.sh
sh src/icrc1/scripts/deploy_bbb.sh


# === SETUP TOKENS ====
sh src/dex/scripts/initialize_local_balance.sh

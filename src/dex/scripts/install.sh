set -e
dfx stop && dfx start --background --clean

### === DEPLOY LOCAL LEDGER =====
dfx identity new minter --disable-encryption || true
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)

### === DEPLOY TOKENS ====
sh src/icrc1/scripts/deploy_aaa.sh
sh src/icrc1/scripts/deploy_bbb.sh


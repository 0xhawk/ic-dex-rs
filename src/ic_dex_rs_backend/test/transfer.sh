set -x
set -e
trap 'catch' ERR
catch() {
    dfx identity use default
    echo "FAIL"
    exit 1
}
dfx identity use default
export PRINCIPAL=$(dfx identity get-principal)
dfx canister call ic_dex_rs_backend clear

# set allowance on DIP20 tokens
export DEX_ID=$(dfx canister id ic_dex_rs_backend)
dfx canister call AkitaDIP20 approve '(principal '\"$DEX_ID\"',10000000)'
dfx canister call GoldenDIP20 approve '(principal '\"$DEX_ID\"',10000000)'
# get ICP deposit address
# export ICP_DEPOSIT_ADDR=$(dfx canister call ic_dex_rs_backend getDepositAddress | tr -d '\n' | sed 's/,)/)/')
# # deposit some ICP in DEX
# dfx canister call ledger transfer "(record { amount = record { e8s = 1000000 }; to = $ICP_DEPOSIT_ADDRESS; fee = record { e8s = 10000 }; memo = 1;})"

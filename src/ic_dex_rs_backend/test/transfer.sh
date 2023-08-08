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

#set allowance on DIP20 tokens
export DEX_ID=$(dfx canister id ic_dex_rs_backend)
dfx canister call AkitaDIP20 approve '(principal '\"$DEX_ID\"',10000000)'
dfx canister call GoldenDIP20 approve '(principal '\"$DEX_ID\"',10000000)'

dfx canister call ic_dex_rs_backend greet "Internet Compunter"
dfx canister call ic_dex_rs_backend owner

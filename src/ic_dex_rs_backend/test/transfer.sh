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
dfx canister call ic_dex_rs_backend greet "Internet Compunter"
dfx canister call ic_dex_rs_backend owner
dfx canister call ic_dex_rs_backend clear
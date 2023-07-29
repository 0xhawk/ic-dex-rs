set -x
set -e
trap 'catch' ERR
catch() {
    dfx identity use default
    echo "FAIL"
    exit 1
}
dfx canister install --mode=reinstall ic_dex_rs_backend
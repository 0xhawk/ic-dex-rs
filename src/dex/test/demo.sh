set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

# create new demo identities
dfx identity new user1 --disable-encryption || true
dfx identity use user1
export USER1_PRINCIPAL=$(dfx identity get-principal)
export USER1_ACC=$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$(dfx ledger account-id)'")]) + "}")')
dfx identity new user2 --disable-encryption || true
dfx identity use user2
export USER2_PRINCIPAL=$(dfx identity get-principal)
export USER2_ACC=$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$(dfx ledger account-id)'")]) + "}")')

dfx identity use default
dfx canister call dex clear
dfx canister call AkitaDIP20 transfer  '(principal '\"$USER1_PRINCIPAL\"',10000000)'

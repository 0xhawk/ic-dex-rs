set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

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
# dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'
# dfx canister install ledger --argument "(variant {Init = record { token_name = \"NAME\"; token_symbol = \"SYMB\"; transfer_fee = 1000000; metadata = vec {}; minting_account = record {owner = principal \"$(dfx --identity minter identity get-principal)\";}; initial_balances = vec {}; archive_options = record {num_blocks_to_archive = 1000000; trigger_threshold = 1000000; controller_id = principal \"$(dfx identity get-principal)\"}; }})"
dfx canister install ledger --argument "(variant {Init = record { token_name = \"NAME\"; token_symbol = \"SYMB\"; transfer_fee = 1000000; metadata = vec {}; minting_account = record {owner = principal \"$(dfx --identity minter identity get-principal)\";}; initial_balances = vec {}; archive_options = record {num_blocks_to_archive = 1000000; trigger_threshold = 1000000; controller_id = principal \"$(dfx identity get-principal)\"}; }})"
dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}; archive_options = opt record { trigger_threshold = 2000; num_blocks_to_archive = 1000; controller_id = principal "'${LEDGER_PRINCIPAL}'" }})'
# transfer ICP tokens to users
# dfx canister call ledger transfer "(record { amount = record { e8s = 100000 }; to = $ALICE_ACC; fee = record { e8s = 10000}; memo = 1;})"
# dfx canister call ledger transfer "(record { amount = record { e8s = 100000 }; to = $BOB_ACC; fee = record { e8s = 10000}; memo = 1;})"
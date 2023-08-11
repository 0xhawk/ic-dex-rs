#!/bin/bash

# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=local

# Change the variable to the principal that can mint and burn tokens.
export MINTER_PRINCIPAL=$(dfx identity get-principal)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="A Token"
export TOKEN_SYMBOL="AAA"

dfx deploy --network ${NETWORK} icrc1-aaa-ledger --argument "(variant { Init = 
      record {
        token_name = \"${TOKEN_NAME}\";
        token_symbol = \"${TOKEN_SYMBOL}\";
        minting_account = record { owner = principal \"${MINTER_PRINCIPAL}\";};
        initial_balances = vec { record { record { owner = principal \"${MINTER_PRINCIPAL}\";}; 2000000000; } };
        metadata = vec {};
        transfer_fee = 10;
        archive_options = record {
          trigger_threshold = 2000;
          num_blocks_to_archive = 1000;
          controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        };
      }
})"

dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_total_supply
dfx canister --network ${NETWORK} call icrc1-aaa-ledger icrc1_balance_of "(record { owner = principal \"${MINTER_PRINCIPAL}\"; })"
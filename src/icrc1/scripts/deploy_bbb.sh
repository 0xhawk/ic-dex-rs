#!/bin/bash
set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}

# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=local

dfx identity use minter

# Change the variable to the principal that can mint and burn tokens.
export MINTER_PRINCIPAL=$(dfx identity get-principal)
export TOKEN_NAME="B-Token"
export TOKEN_SYMBOL="BBB"

dfx deploy --network ${NETWORK} icrc1-bbb-ledger --argument "(variant { Init = 
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
          controller_id = principal \"${MINTER_PRINCIPAL}\";
        };
      }
})"
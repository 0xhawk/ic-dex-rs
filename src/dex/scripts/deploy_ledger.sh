#!/bin/bash

# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=local

# Change the variable to the principal that can mint and burn tokens.
export MINTER_PRINCIPAL=$(dfx identity get-principal)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="My Token"
export TOKEN_SYMBOL=XMTK

dfx deploy --network ${NETWORK} icrc1-ledger --argument "(variant {Init = record {
  token_name = opt \"${TOKEN_NAME}\";
  token_symbol = opt \"${TOKEN_SYMBOL}\";
  minting_account = \"${MINTER_PRINCIPAL}\";
  initial_values = vec {};
  send_whitelist = vec {};
  archive_options = opt record {
    trigger_threshold = 2000;
    num_blocks_to_archive = 1000;
    controller_id = principal \"${ARCHIVE_CONTROLLER}\";
    cycles_for_archive_creation = opt 10_000_000_000_000;
  }
}})"
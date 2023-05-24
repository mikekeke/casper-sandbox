#!/usr/bin/env bash

set -euo pipefail

PUB_HEX="../../test-1-ed25519-keys/public_key_hex"
NODE_ADDR="http://94.130.10.55:7777"

ACCOUNT_HASH=$(casper-client account-address --public-key ${PUB_HEX})

echo $ACCOUNT_HASH

ROOT_HASH=$(casper-client get-state-root-hash --node-address ${NODE_ADDR} | jq -r ".result.state_root_hash")

echo $ROOT_HASH

casper-client query-state \
  --node-address ${NODE_ADDR} \
  --state-root-hash ${ROOT_HASH} \
  --key deploy-45c7217a6b7e0a23ffb8b2a44a002c966fec389b4cc7121d3522b80a9a9fe664
  # --key deploy-$1
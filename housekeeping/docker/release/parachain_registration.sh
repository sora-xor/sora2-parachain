#!/bin/bash
set -ex

api=/usr/local/bin/polkadot-js-api
parachain=/usr/local/bin/parachain-collator

$parachain export-genesis-wasm > /opt/sora2/genesis.wasm
$parachain export-genesis-state > /opt/sora2/genesis.state

api_command_template="--ws $RELAYCHAIN_API_ENDPOINT --sudo --seed"
api_runtime_template="{\"scheduling\":\"Always\"} @/opt/sora2/genesis.wasm `cat /opt/sora2/genesis.state`"

function api_query() {
    $api \
    $api_command_template "$MNEMO_PHRASE" \
    $1 \
    $PARACHAIN_ID \
    $2
}

api_query "sudoScheduleParaInitialize" "$api_runtime_template"
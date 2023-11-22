#!/bin/bash
set -e

if [ "$pr" = true ] && [ "$prBranch" != "master" ]; then
    printf "👷‍♂️ starting clippy \n"
    SKIP_WASM_BUILD=1 cargo clippy --features kusama --message-format=json > clippy_kusama_report.json
    SKIP_WASM_BUILD=1 cargo clippy --features polkadot --message-format=json > clippy_polkadot_report.json
    SKIP_WASM_BUILD=1 cargo clippy --features rococo,runtime-benchmarks --message-format=json > clippy_rococo_report.json
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy --message-format=json -- -D warnings > clippy_report.json || exit 0
fi
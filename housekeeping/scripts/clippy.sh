#!/bin/bash
set -e

if [ "$pr" = true ] && [ "$prBranch" != "master" ]; then
    printf "👷‍♂️ starting clippy \n"
    SKIP_WASM_BUILD=1 cargo clippy --features kusama
    SKIP_WASM_BUILD=1 cargo clippy --features polkadot
    SKIP_WASM_BUILD=1 cargo clippy --features rococo,runtime-benchmarks
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy --message-format=json -- -D warnings > clippy_report.json || exit 0
fi
#!/bin/bash
set -e

clippycommand="SKIP_WASM_BUILD=1 cargo clippy --features"
clippyfeatures=("kusama" "polkadot" "rococo,runtime-benchmarks")

if [ "$pr" = true ] && [ "$prBranch" != "master" ]; then
    for clippyfeature in "${clippyfeatures[@]}"; do
        printf "👷‍♂️ starting clippy with $clippyfeature feature \n"
        $clippycommand $clippyfeature
    done
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy -- -D warnings || exit 0
fi
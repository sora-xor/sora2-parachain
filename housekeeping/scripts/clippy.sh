#!/bin/bash
set -e

clippycommand="cargo clippy --features"
clippyfeatures=("kusama" "polkadot" "rococo,runtime-benchmarks")

if [ "$pr" = true ] && [ "$prBranch" != "master" ]; then
    for clippyfeature in "${clippyfeatures[@]}"; do
        printf "👷‍♂️ starting clippy with $clippyfeature feature \n"
        export SKIP_WASM_BUILD=1 && $clippycommand $clippyfeature
    done
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy -- -D warnings || exit 0
fi
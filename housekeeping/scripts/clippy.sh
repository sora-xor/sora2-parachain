#!/bin/bash
set -e

clippycommand="cargo clippy --features"
clippyfeatures=("kusama" "polkadot" "rococo,runtime-benchmarks")

if [ "$pr" = true ] && [ "$prBranch" != "master" ]; then
    for clippyfeature in "${clippyfeatures[@]}"; do
        printf "👷‍♂️ starting clippy with $clippyfeature feature \n"
        export SKIP_WASM_BUILD=1 && $clippycommand $clippyfeature --message-format=json > clippy_report.json
    done
else
    printf "👷‍♂️ starting a regular clippy \n"
    cargo clippy --message-format=json -- -D warnings > clippy_report.json || exit 0
fi
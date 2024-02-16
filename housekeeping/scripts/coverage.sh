#!/bin/sh

export RUSTFLAGS="-Cinstrument-coverage"
export SKIP_WASM_BUILD=1
export LLVM_PROFILE_FILE="sora2-%p-%m.profraw"

grcov . --binary-path ./target/debug -s . -t lcov --branch -o ./lcov_report --ignore-not-existing --ignore  "/opt/cargo/**" "target/debug" "node/src" --log-level="ERROR" --llvm-path='/usr/lib/llvm-14/bin'

find . -type f -name '*.profraw' -delete
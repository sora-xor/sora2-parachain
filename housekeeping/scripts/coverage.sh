#!/bin/sh

grcov . --binary-path ./target/release -s . -t lcov --branch -o ./lcov_report --ignore-not-existing --ignore  "/opt/cargo/**" "target/release" "node/src" --log-level="ERROR" --llvm-path='/usr/lib/llvm-15/bin'

find . -type f -name '*.profraw' -delete
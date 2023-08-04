#!/bin/bash
set -e

buidcmd="cargo b -r --features"
testcmd="cargo test -r --features"
networks=(kusama rococo polkadot)
wasm_in="./target/release/wbuild/sora2-parachain-runtime_"

rm -rf ~/.cargo/registry/

for network in ${networks[@]}
do
 printf "🏗️ Build "$network" will start now... \n"
 $buidcmd "$network"
 $testcmd "$network"
 wasm_out=./sora2-parachain-runtime_$network.compact.wasm
 mv "$wasm_in$network.compact.wasm" "$wasm_out"
 if [ -f "$wasm_out" ]; then
    printf "✅ "$wasm_out" OK\n"
 else
    exit 1
 fi
done

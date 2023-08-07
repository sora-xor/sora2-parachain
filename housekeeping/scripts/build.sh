#!/bin/bash
set -e

buidcmd="cargo b -r --features"
testcmd="cargo test -r --features"
networks=(kusama rococo polkadot)

rm -rf ~/.cargo/registry/

for network in ${networks[@]}
do
 printf "🏗️ Build "$network" will start now... \n"
 $buidcmd "$network"
 $testcmd "$network"
 wasm_in="./target/release/wbuild/$network-runtime/"
 wasm_out=./sora2-parachain-runtime_$network.compressed.wasm
 wasm_file=$(ls "$wasm_in" | grep "$network" | grep ".compressed.wasm")
 mv "$wasm_in$wasm_file" "$wasm_out"
 if [ -f "$wasm_out" ]; then
    printf "✅ "$wasm_out" OK\n"
 else
    exit 0
 fi
done

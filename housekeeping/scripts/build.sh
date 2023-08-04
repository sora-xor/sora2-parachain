#!/bin/bash
set -e

parachains=(kusama rococo polkadot)

rm -rf ~/.cargo/registry/

for parachain in ${parachains[@]}
do
 printf "🏗️ Building $parachain will starting now... %s/n"
 cargo b -r "$parachain"
 cargo test -r "$parachain"
 wasm_out=./sora2-parachain-runtime_$parachain.compact.wasm
 mv ./target/release/wbuild/sora2-parachain-runtime_$parachain.compact.wasm $wasm_out
 if [ -f "$wasm_out" ]; then
    printf "✅ $wasm_out OK %/n"
 else
    exit 1
 fi
done

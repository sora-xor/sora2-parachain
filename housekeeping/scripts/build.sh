#!/bin/bash
set -e

buidcmd="cargo b -r --features"
testcmd="cargo test -r --features"
networks=(kusama rococo polkadot)

rm -rf ~/.cargo/registry/


for network in ${networks[@]}
do
   if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
      printf "🏗️ Building of "$network" will start now... \n"
      $buidcmd "$network"
      $testcmd "$network"
      wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
      wasm_out=./sora2-parachain-runtime_$network.compact.compressed.wasm
      wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
      mv "$wasm_in$wasm_file" "$wasm_out"
   else
      printf "⚡️ There is no tag here, only tests run. \n"
      printf "🏃 Running tests for "$network"... \n"
      $testcmd "$network"
      wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
      wasm_out=./sora2-parachain-runtime_$network.compact.compressed.wasm     
      wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
      mv "$wasm_in$wasm_file" "$wasm_out"
   fi
   if [ -f "$wasm_out" ]; then
      printf "✅ "$wasm_out" found!\n"
   else
      printf "❌"$wasm_out" can't found!\n"
      exit 1
   fi
done

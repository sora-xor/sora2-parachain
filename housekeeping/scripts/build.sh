#!/bin/bash
set -e

benchmarkcmd="cargo build --release --locked --features runtime-benchmarks,kusama --bin parachain-collator"
buidcmd="cargo b -r --features"
testcmd="cargo test -r --features"
networks=(kusama rococo polkadot)
binaryfile="target/release/parachain-collator"
binaryfilepath="housekeeping/parachain-collator"
errorfile="benchmarking_errors.txt"

rm -rf ~/.cargo/registry/

if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
   printf "🕙 Building benchmarks will start now... \n"
   $benchmarkcmd
fi

for network in ${networks[@]}
do
   if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]] && [[ ${TAG_NAME} != 'benchmarking'* ]]; then
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

if [ -f "$binaryfile" ]; then
   cp "$binaryfile" "$binaryfilepath"
fi

if [ -f "$errorfile" ]; then
   printf "⚠️ build failed, please check the error below\n"
   cat "$errorfile"
fi
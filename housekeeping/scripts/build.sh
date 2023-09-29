#!/bin/bash
set -e

benchmarkcmd="cargo build --release --locked --features runtime-benchmarks,kusama --bin parachain-collator"
buidcmd="cargo b -r --features"
testcmd="cargo test -r --features"
benchfeature="runtime-benchmarks"
networks=(kusama polkadot rococo)
binaryfile="target/release/parachain-collator"
binaryfilepath="housekeeping/parachain-collator"
errorfile="benchmarking_errors.txt"
wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
wasm_out=./sora2-parachain-runtime_$buildfeature.compact.compressed.wasm
wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")

rm -rf ~/.cargo/registry/

if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
   if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
         buildcmd="cargo build --release --locked --bin parachain-collator --features" 
         buildfeature="runtime-benchmarks,kusama"
   elif [[ $buildTag = 'dev' ]] || [[ $buildTag =~ 'stage'* ]]; then
         buildfeature="rococo"
   elif [[ ${TAG_NAME} =~ 'kusama'* ]]; then
         buildfeature="kusama"
   elif [[ ${TAG_NAME} =~ 'polkadot'* ]]; then 
         buildfeature="polkadot"
   fi
   printf "🕙 Building with feature $buildfeature will start now... \n"
   $testcmd "$buildfeature"
   $buidcmd "$buildfeature"
   mv "$wasm_in$wasm_file" "$wasm_out"
   if [ -f "$wasm_out" ]; then
      printf "✅ "$wasm_out" found!\n"
   else
      printf "❌"$wasm_out" can't found!\n"
      exit 1
   fi
else
   for network in ${networks[@]}
   do 
      printf "⚡️ There is no tag here, only tests run. \n"
      printf "🏃 Running tests for "$network"... \n"
      $testcmd "$network" "$benchfeature"
      wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
      wasm_out=./sora2-parachain-runtime_$network.compact.compressed.wasm     
      wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
      mv "$wasm_in$wasm_file" "$wasm_out"
   done
fi

if [ -f "$binaryfile" ]; then
   cp "$binaryfile" "$binaryfilepath"
fi

if [ -f "$errorfile" ]; then
   printf "⚠️ build failed, please check the error below\n"
   cat "$errorfile"
fi

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

test() {
  if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
    if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
        buildfeature="runtime-benchmarks,kusama"
    elif [[ ${TAG_NAME} = 'kusama-'* ]]; then
        buildfeature="kusama"
    elif [[ ${TAG_NAME} = 'polkadot-'* ]]; then
        buildfeature="polkadot"
    elif [[ ${TAG_NAME} = 'alphanet-'* ]] || [[ ${TAG_NAME} = 'stage-alphanet-'* ]] ; then
        buildfeature="alphanet"
    elif [[ -n $buildTag ]] || [[ ${TAG_NAME} = 'stage-'* ]] || [[ ${TAG_NAME} = 'test-'* ]]; then
        buildfeature="rococo"
    fi
    printf " Testing for feature $buildfeature will start now... \n"
    $testcmd "$buildfeature"
    wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
    wasm_out=./sora2-parachain-runtime_$buildfeature.compact.compressed.wasm
    wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
    mv "$wasm_in$wasm_file" "$wasm_out"
    if [ -f "$wasm_out" ]; then
      printf "✅ "$wasm_out" found!\n"
    else
      printf "❌"$wasm_out" can't found!\n"
      exit 1
    fi
  else
    # No buildtags. Only coverage
    export RUSTFLAGS="-Cinstrument-coverage"
    export LLVM_PROFILE_FILE="sora2-%p-%m.profraw"
    for network in ${networks[@]}
    do 
      printf "🕙 No buildtags. Running coverage tests for $network feature... \n"
      $testcmd "$network" "$benchfeature"
      wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
      wasm_out=./sora2-parachain-runtime_$network.compact.compressed.wasm     
      wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
      mv "$wasm_in$wasm_file" "$wasm_out"
      if [ -f "$wasm_out" ]; then
         printf "✅ "$wasm_out" found!\n"
      else
         printf "❌"$wasm_out" can't found!\n"
         exit 1
      fi
    done
  fi
}

build() {
  if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
    if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
      buildcmd="cargo build --release --locked --bin parachain-collator --features"
      buildfeature="runtime-benchmarks,kusama"
    elif [[ ${TAG_NAME} = 'kusama-'* ]]; then
      buildfeature="kusama"
    elif [[ ${TAG_NAME} = 'polkadot-'* ]]; then
      buildfeature="polkadot"
    elif [[ -n $buildTag ]] || [[ ${TAG_NAME} = 'stage-'* ]] || [[ ${TAG_NAME} = 'test-'* ]]; then
      buildfeature="rococo"
    fi
    printf "🕙 Building with feature $buildfeature will start now... \n"
    $buidcmd "$buildfeature"
    wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
    wasm_out=./sora2-parachain-runtime_$feature.compact.compressed.wasm
    wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
    mv "$wasm_in$wasm_file" "$wasm_out"
    if [ -f "$wasm_out" ]; then
        printf "✅ $wasm_out found!\n"
    else
        printf "❌ $wasm_out can't found!\n"
        exit 1
    fi
  fi
}

if [ -f "$binaryfile" ]; then
   cp "$binaryfile" "$binaryfilepath"
fi

if [ -f "$errorfile" ]; then
   printf "⚠️ build failed, please check the error below\n"
   cat "$errorfile"
fi

if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists in this workflow. Skipped."
fi

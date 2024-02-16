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

rm -rf ~/.cargo/registry/

# build func with feature
build() {
    feature=$1
    printf "🕙 Building with feature $feature will start now... \n"
    $buidcmd "$feature"
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
}

# test func with tag
releasetest() {
    feature=$1
    printf "🕙 Testing with feature $feature will start now... \n"
    $testcmd "$feature"
}

# test func without tag
test() {
    export RUSTFLAGS="-Cinstrument-coverage"
    export SKIP_WASM_BUILD=1
    export LLVM_PROFILE_FILE="sora2-%p-%m.profraw"
    printf "⚡️ There is no tag here, only tests run. \n"  
    for network in ${networks[@]}
    do 
        printf "🏃 Running tests for $network... \n"
        $testcmd "$network" "$benchfeature"
        wasm_in="./target/release/wbuild/sora2-parachain-runtime/"
        wasm_out=./sora2-parachain-runtime_$network.compact.compressed.wasm     
        wasm_file=$(ls "$wasm_in" | grep ".compact.compressed.wasm")
        mv "$wasm_in$wasm_file" "$wasm_out"
        if [ -f "$wasm_out" ]; then
            printf "✅ $wasm_out found!\n"
        else
            printf "❌ $wasm_out can't found!\n"
            exit 1
        fi
    done
}

# build workflow
if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
    if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
      buildcmd="cargo build --release --locked --bin parachain-collator --features"
      releasetest "runtime-benchmarks,kusama"
      build "runtime-benchmarks,kusama"
    elif [[ $buildTag = 'dev' || $buildTag = 'latest' ]] || [[ ${TAG_NAME} = 'stage-'* ]] || [[ ${TAG_NAME} = 'test-'* ]]; then
      releasetest "rococo"  
      build "rococo"
    elif [[ ${TAG_NAME} = 'kusama-'* ]]; then
      releasetest "kusama"
      build "kusama"
    elif [[ ${TAG_NAME} = 'polkadot-'* ]]; then
      releasetest "polkadot"
      build "polkadot"
    fi
else
    test
fi

if [ -f "$binaryfile" ]; then
   cp "$binaryfile" "$binaryfilepath"
fi

if [ -f "$errorfile" ]; then
   printf "⚠️ build failed, please check the error below\n"
   cat "$errorfile"
fi

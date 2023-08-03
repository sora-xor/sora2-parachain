#!/bin/bash
rm -rf ~/.cargo/registry/
cargo test -r
cargo build --release
cp target/release/parachain-collator housekeeping/parachain-collator
mv ./target/release/wbuild/sora2-parachain-runtime/sora2-parachain-runtime.compact.compressed.wasm ./sora2-parachain-runtime.compact.compressed.wasm
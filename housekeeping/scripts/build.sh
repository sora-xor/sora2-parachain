#!/bin/bash
rm -rf ~/.cargo/registry/
cargo test -r
cargo build --release
cp target/release/parachain-collator housekeeping/parachain-collator
mv ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm ./parachain_template_runtime.compact.compressed.wasm

kusama:
	cargo build --release --features kusama

polkadot:
	cargo build --release --features polkadot

rococo:
	cargo build --release --features rococo

test-kusama:
	cargo test --release --features kusama

test-polkadot:
	cargo test --release --features polkadot

test-rococo:
	cargo test --release --features rococo

test-all:
	cargo test --release -p sora2-parachain-runtime --features rococo
	cargo test --release -p sora2-parachain-runtime --features kusama
	cargo test --release -p sora2-parachain-runtime --features polkadot

lint:
	SKIP_WASM_BUILD=1 cargo clippy --features rococo

meta:
	cargo build --release -p parachain-gen --features rococo
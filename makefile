
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

lint:
	cargo clippy --all-targets
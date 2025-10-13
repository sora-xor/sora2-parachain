
acala build-spec --chain karura-local --disable-default-bootnode > karura.json
acala export-genesis-state --chain karura-local > 2000-genesis
acala export-genesis-wasm --chain karura-local > 2000-genesis-wasm
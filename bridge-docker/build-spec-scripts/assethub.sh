polkadot-parachain build-spec --chain asset-hub-rococo-local --disable-default-bootnode > assethub.json
polkadot-parachain export-genesis-state --chain asset-hub-rococo-local > 1000-genesis
polkadot-parachain export-genesis-wasm --chain asset-hub-rococo-local > 1000-genesis-wasm
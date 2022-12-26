
parachain-collator build-spec --chain docker-local > sora.json
parachain-collator export-genesis-state --chain docker-local > 2011-genesis
parachain-collator export-genesis-wasm --chain docker-local > 2011-genesis-wasm
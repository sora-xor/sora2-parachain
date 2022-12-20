
parachain-collator build-spec --chain local > sora.json
parachain-collator export-genesis-state --chain local > 2011-genesis
parachain-collator export-genesis-wasm --chain local > 2011-genesis-wasm
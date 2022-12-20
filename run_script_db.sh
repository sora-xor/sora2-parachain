#!/bin/bash

export RUST_LOG=beefy_light_client=trace,bridge_common=trace,substrate_bridge_channel=trace,info

mkdir -p run
../relay/polkadot/target/release/polkadot build-spec --chain rococo-local --raw > ./run/rococo-local.json
./target/release/parachain-collator export-genesis-state --chain local > ./run/genesis
./target/release/parachain-collator export-genesis-wasm --chain local > ./run/genesis-wasm
../relay/polkadot/target/release/polkadot --chain rococo-local --alice --port 40333 --ws-port 10944 --rpc-port 10954 --tmp > /tmp/rococo-10944.log 2>&1 &
../relay/polkadot/target/release/polkadot --chain rococo-local   --bob --port 40334 --ws-port 10945 --rpc-port 10955 --tmp > /tmp/rococo-10945.log 2>&1 &

./target/release/parachain-collator --execution native --pruning=archive --enable-offchain-indexing true --alice   --collator --tmp --port 40433 --ws-port 10844 --rpc-cors all --rpc-port 10854 --chain local -- --execution native --chain ./run/rococo-local.json --port 40533 --ws-port 10744 > /tmp/parachain-10844.log 2>&1 &
./target/release/parachain-collator --execution native --pruning=archive --enable-offchain-indexing true --bob     --collator --tmp --port 40434 --ws-port 10845 --rpc-cors all --rpc-port 10855 --chain local -- --execution native --chain ./run/rococo-local.json --port 40534 --ws-port 10745 > /tmp/parachain-10845.log 2>&1 &
./target/release/parachain-collator --execution native --pruning=archive --enable-offchain-indexing true --charlie --collator --tmp --port 40435 --ws-port 10846 --rpc-cors all --rpc-port 10856 --chain local -- --execution native --chain ./run/rococo-local.json --port 40535 --ws-port 10746 > /tmp/parachain-10846.log 2>&1 &

wait

sleep 999999

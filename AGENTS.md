**SORA Parachain Guide**

- **Networks**: This repository builds the SORA parachain for multiple relay chains via cargo features and chain-specs.
  - **SORA Kusama**: `para_id` 2011, `ss58Format` 420, `tokenSymbol` `XOR`, `tokenDecimals` 18. Chain spec: `node/res/kusama.json`.
  - **SORA Polkadot**: `para_id` 2025, `ss58Format` 81, `tokenSymbol` `XOR`, `tokenDecimals` 18. Chain spec: `node/res/polkadot.json`.
  - **Other specs**: `node/res/rococo.json`, `node/res/alphanet.json` for testing.

**Chain Specs**

- **Files**: `node/src/chain_spec.rs` loads JSON specs and provides `coded_config(...)` for generated configs.
- **Relay selection**: `RelayChain::{Kusama,Polkadot,...}` determines name, id, root key, session keys, and `bridge_network_id`.
- **Genesis**: `testnet_genesis(...)` sets WASM code, endowed balances, collator session keys, safe XCM version, BEEFY light client, council/technical committee/democracy, and parachain id.
- **CLI presets**: `parachain-collator --chain kusama|polkadot|rococo|alpha` or provide a path to a JSON spec. See `node/src/command.rs` `load_spec`.

**Runtime Overview**

- **Pallet set**: Collation (Aura, Session, CollatorSelection), Balances/TransactionPayment, XCM (`pallet_xcm`, `cumulus_pallet_xcmp_queue`, `cumulus_pallet_dmp_queue`, `orml_xtokens`), Governance (`Council`, `TechnicalCommittee`, `ElectionsPhragmen`, `Democracy`, `Preimage`, `Scheduler`, `Utility`).
- **Sudo**: Included only under `rococo`/`alphanet` features. Kusama/Polkadot builds exclude sudo.
- **Versioning**: `runtime/src/lib.rs` sets `spec_name`/`spec_version` and `SS58Prefix` based on feature flags.
- **XCM config**: `runtime/src/xcm_config.rs` defines `XcmRouter` (Parent UMP + XCMP), `IsReserve` via `MultiNativeAsset<AbsoluteReserveProvider>`, `LocalAssetTransactor` using the `XCMApp` pallet, and `SAFE_XCM_VERSION` from `node/src/chain_spec.rs`.

**Bridges to SORA Mainnet**

- **Light client**: `BeefyLightClient` pallet configured with `bridge_types::SubNetworkId` for the running network; provides sidechain randomness and verification plumbing.
- **Channels**: `substrate_bridge_channel::{inbound,outbound}` pallets are included. `XCMApp` uses `SubstrateBridgeOutboundChannel` as its outbound.
- **Dispatch**: Bridge calls are funneled through `dispatch` pallet with `MultisigVerifier` and `bridge_data_signer` for authorization paths.
- **XCM App**: `pallets/xcm-app` integrates asset transfers and message routing; `XCMSenderWrapper` forwards to `PolkadotXcm`.

**Governance**

- **Collectives**: `Council` and `TechnicalCommittee` are included and initialized from the chain-spec. Elections via `ElectionsPhragmen`.
- **Democracy**: Enabled for proposals and referenda; `Preimage` and `Scheduler` support governance execution.
- **Root**: On Kusama/Polkadot, privileged actions flow through governance origins rather than `sudo`.

**XCM and Channels**

- **Routing**: Upward messages via UMP to the relay chain; lateral via XCMP (`XcmpQueue`).
- **Reserve handling**: The runtime trusts native assets via `IsReserve = MultiNativeAsset<AbsoluteReserveProvider>`. Adjust this when migrating KSM/DOT reserve to Asset Hub.
- **HRMP**: Channels to system parachains (e.g., Asset Hub) are opened by sending an XCM to the relay chain requesting an HRMP open; system parachains auto-accept under current policies.

**Developer Notes**

- **Build**: `cargo build --release --features kusama` or `--features polkadot` to target each network.
- **Launch locally**: See `polkadot-launch/config.json` and `bridge-docker/` for local/devnet topologies and spec generation scripts.
- **Common paths**: Runtime (`runtime/src/*.rs`), XCM config (`runtime/src/xcm_config.rs`), chain-spec (`node/src/chain_spec.rs`), specs (`node/res/*.json`).


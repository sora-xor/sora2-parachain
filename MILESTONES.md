**SORA Kusama Milestones**

- **Objective**: Migrate KSM reserve from Relay Chain to Asset Hub, open HRMP channel to Kusama Asset Hub, and coordinate state moves and UIs. Based on “Changing the DOT reserve from Relay Chain to Asset Hub” (adapted for KSM) and the Polkadot tutorial for opening HRMP channels to system parachains.

**Phase 1: Analysis & Design**

- **Inventory**: Confirm current XCM reserve handling in `runtime/src/xcm_config.rs` (`IsReserve = MultiNativeAsset<AbsoluteReserveProvider>`), XCM versioning, and any custom reserve logic.
- **Target reserve**: Use Kusama Asset Hub (`ParaId 1000`) as the reserve location for KSM; remove Relay Chain as reserve for KSM.
- **Bridging impact**: Verify that bridge pallets (`BeefyLightClient`, `substrate_bridge_channel`, `XCMApp`) are unaffected by reserve change.

**Phase 2: Runtime Change**

- **Add KSM-from-Asset-Hub guard**: Implement a `ContainsPair<Asset, Location>` that allows KSM identified by the Relay (parents=1, interior=Here) with reserve location `parents=1, interior=X1(Parachain(1000))`; remove/avoid trusting the Relay as KSM reserve. Example outline:
  - Add `DotFromAssetHub`-style guard (rename to `KsmFromAssetHub`) and include it in the `IsReserve` tuple.
  - Ensure `UniversalLocation`/`SelfLocation` and `LocationToAccountId` remain consistent.
- **Migrations**: If any pallet stores assumptions about reserve locations, add a lightweight runtime migration to update them.
- **Bumps**: Increment `spec_version` in `runtime/src/lib.rs` for Kusama feature build.

**Phase 3: Testing**

- **Unit/XCM tests**: Extend `runtime/src/xcm_tests` to assert that:
  - KSM reserve matches Asset Hub and that Relay-origin KSM with Asset Hub reserve is accepted.
  - Teleports remain disabled; reserve transfers still allowed as configured.
- **try-runtime**: Build with `--features try-runtime` and run on a recent SORA Kusama state snapshot to dry-run migration.
- **Ecosystem tests**: Add/adjust cases in `polkadot-ecosystem-tests` to validate pre/post AHM behavior on Kusama.

**Phase 4: Governance & Upgrade**

- **Preimage**: Submit preimage for the new runtime Wasm.
- **Democracy**: Open referendum to schedule the runtime upgrade; include a scheduler item if needed.
  - Minimum deposit on SORA Kusama: 1 XOR (1_000_000_000_000_000_000 plancks), see `DemocracyMinimumDeposit` in `runtime/src/lib.rs`.
  - Preimage deposits (pallet-preimage): base 1 and byte 1 (plancks), configured in `PreimageBaseDeposit` and `PreimageByteDeposit` (negligible).
- **Fast-track (optional)**: Use Council/Technical Committee to expedite if time-sensitive.
- **Rollout**: On enactment, validators/collators upgrade and begin producing with the new `spec_version`.

**Phase 5: HRMP to Asset Hub (Kusama)**

- **Channel open (one-sided)**: From SORA Kusama (para 2011), submit a single XCM to the Relay to request HRMP open to Asset Hub (para 1000). System parachains auto-accept; a single message is sufficient according to the Polkadot docs for para-to-system channels.
  - Parameters: `max_capacity`, `max_message_size` sized per traffic expectations.
  - Submission: Use `pallet_xcm::send` from Root origin to the Relay with `HrmpInitiateOpenChannel` (and, if needed by policy, `HrmpAcceptOpenChannel`).
  - Fees on Relay (KSM): Estimate the fee for `hrmp.initOpenChannel` and set `BuyExecution.fees` in the XCM to at least 2x the estimate. Use:
    - `cd tools/js && node --loader ts-node/esm ./tmp/open_to_asset_hub.ts --relay-ws wss://kusama-rpc.polkadot.io --para 1000 --capacity 1000 --messageSize 1048576 --estimate`
    - Relay HRMP fee (no tip): 464995403 plancks (~0.000464995403 KSM)
    - Set BuyExecution.fees to at least 929,990,806 plancks (~0.000929990806 KSM)

**Phase 5b: HRMP to Coretime (Kusama) for Auto-Renewal**

- Objective: Open an HRMP channel from SORA Kusama (para 2011) to the Kusama Coretime system parachain to enable automatic coretime renewals, per the Coretime Renewal guide.
- Steps:
  - Identify the Kusama Coretime parachain ID (check Polkadot Developer Docs or chain state; it is a system parachain and auto-accepts channels).
  - Submit a single XCM to the Relay with `hrmp.initOpenChannel(<CORETIME_PARA_ID>, max_capacity, max_message_size)` wrapped in `Transact`, same as Asset Hub.
  - Recommended parameters: `max_capacity = 1000`, `max_message_size = 1_048_576` (1 MiB), adjust per expected usage.
  - Fees on Relay (KSM): Estimate exactly with the HRMP script (replace PARA with Coretime ID):
    - `cd tools/js && node --loader ts-node/esm ./tmp/open_to_asset_hub.ts --relay-ws wss://kusama-rpc.polkadot.io --para <CORETIME_PARA_ID> --capacity 1000 --messageSize 1048576 --estimate`
    - Set `BuyExecution.fees` in the XCM to at least 2× the printed plancks.
- After channel opens, configure your renewal flow per the Coretime Renewal guide (e.g., set renewal parameters and funding on Coretime chain; ensure messages can be delivered over HRMP).
- Reference: Coretime Renewal — https://docs.polkadot.com/develop/parachains/deployment/coretime-renewal/
- **Verification**: Confirm inbound/outbound channel status on both sides; ensure `XcmpQueue` shows traffic and message delivery.

**Phase 6: Sovereign Account Move**

- **Move KSM**: From Root, send an XCM to the Relay that withdraws KSM from the parachain’s sovereign account and re-deposits to the parachain’s sovereign account on Asset Hub. Include `BuyExecution` for fees and route to Asset Hub.
- **Reconciliation**: Verify balances on Relay sovereign and Asset Hub sovereign accounts; update any monitoring alerts or dashboards.

**Phase 7: Coordination & Docs**

- **Wallets/UI**: Coordinate with explorers, wallets, and dApps to route KSM via Asset Hub after the upgrade. Update transfer flows and XCM calls.
- **Docs**: Reflect reserve change in `AGENTS.md` and any public documentation.
- **Monitoring**: Track HRMP channel health, XCM error rates, and user reports.

**Phase 8: Timeline & Owners**

- **T0 (Design complete)**: Sign-off on runtime diff and tests.
- **T0 + 1w (Gov submission)**: Submit preimage and referendum; socialize timeline with ecosystem.
- **T0 + 2–3w (Enactment)**: Runtime upgrade enactment; immediately open HRMP channel and move sovereign KSM.
- **Post (1–2w)**: Monitoring period; finalize documentation and deprecate Relay-reserve assumptions.

**References**

- Changing the reserve to Asset Hub (DOT; adapt for KSM): https://hackmd.io/@n9QBuDYOQXG-nWCBrwx8YQ/HkYVQFS8ke
- Opening HRMP channels to system parachains: https://docs.polkadot.com/tutorials/interoperability/xcm-channels/para-to-system/

**SORA Polkadot Milestones**

- **Objective**: Move DOT reserve handling from the Polkadot Relay Chain to Polkadot Asset Hub (Statemint) as of [runtimes v2.0.2](https://github.com/polkadot-fellows/runtimes/releases/tag/v2.0.2), open HRMP to Asset Hub, and validate staking/compatibility fixes introduced in that release.

**Phase 1: Analysis & Design**

- **Inventory**: Audit `runtime/src/xcm_config.rs` to ensure `DotFromAssetHub` mirrors the new `AssetHubNativeAsset` guard, `IsReserve` trusts Asset Hub, and `LocalAssetTransactor`/`SAFE_XCM_VERSION` remain aligned with Polkadot (XCM v3).
- **Target reserve**: Use Polkadot Asset Hub (`ParaId 1000`, `statemint-2000002`) as the reserve location for DOT; stop accepting Relay-only reserves.
- **Bridging impact**: Confirm bridge pallets (`BeefyLightClient`, `substrate_bridge_channel`, `XCMApp`) do not assume relay-resident DOT balances.

**Phase 2: Runtime Change**

- **Asset guard**: Include `DotFromAssetHub` in the Polkadot `Reserves` tuple so relay-native DOT is only trusted when the reserve location resolves to Asset Hub paths (`X1`, `X2`, `X3` junctions that terminate at Parachain 1000 / GeneralIndex(0)).
- **Version bump**: Increment `spec_version` for the Polkadot build so governance can enact the upgrade and wallets pick up the DOT reserve migration.
- **Release alignment**: Track Statemint `core_version statemint-2000002` for compatibility; ensure metadata/extrinsics remain V14 so parachain messaging to Asset Hub 2.0.2 succeeds.

**Phase 3: Testing**

- **Unit/XCM tests**: Extend `runtime/src/xcm_tests` with `dot_from_asset_hub_reserve_rule` covering relay-native DOT, canonical Asset Hub IDs, and reserve location guards.
- **try-runtime**: Run `cargo try-runtime --features polkadot on-runtime-upgrade` with a recent SORA Polkadot state snapshot to verify storage invariants.
- **Asset Hub handshake**: Dry-run DOT reserve transfers using `orml_xtokens` + XCM simulator to confirm `IsReserve` accepts Asset Hub DOT and rejects Relay-only DOT.

**Phase 4: Governance & Upgrade**

- **Preimage**: Submit the new Wasm that includes `DotFromAssetHub` + spec bump as a democracy preimage on SORA Polkadot.
- **Referendum**: Launch democracy proposal (minimum deposit 1 XOR) and schedule enactment. Use Council/Technical Committee fast-track if Asset Hub alignment is urgent.
- **Communication**: Announce Asset Hub migration timeline to wallets/exchanges so DOT routes shift away from Relay sovereign accounts.

**Phase 5: HRMP to Asset Hub (Polkadot) — Completed**

- **Channel open**: ✔️ Verified on 2025-11-19 via `npx @polkadot/api-cli --ws wss://rpc.polkadot.io query.hrmp.hrmpChannels '[2025,1000]'`, which reports `maxCapacity=1000`, `maxMessageSize=102_400`, and `mqcHead=0x6661…b48` for the `(2025 → 1000)` channel, confirming the HRMP lane to Polkadot Asset Hub is active.
- **Funding**: Use `scripts/hrmp/open_to_asset_hub.ts --relay-ws wss://rpc.polkadot.io --para 1000 ...` to build the WithdrawAsset → BuyExecution → Transact flow with DOT fees. Set `BuyExecution.fees` to ≥2× the relay estimate for any future channel adjustments.
- **Monitoring**: Continue watching `hrmp.hrmpChannels([2025,1000])` and Asset Hub events to ensure throughput/limits remain aligned with demand.

**Phase 5b: HRMP to Coretime (Polkadot) for Auto-Renewal**

- **Objective**: Mirror the Kusama flow by opening a channel from SORA Polkadot (para 2025) to the Polkadot Coretime system parachain (check chain state; currently para 1001) so the automatic coretime renewal process described in [the Coretime guide](https://docs.polkadot.com/develop/parachains/deployment/coretime-renewal/) can operate over HRMP.
- **Channel request**: Use the HRMP helper with `--para <coretime_para_id>` to send `hrmp.initOpenChannel` (capacity 1000, message size 1_048_576) from Root. System parachains auto-accept; ensure DOT fees are withdrawn via WithdrawAsset + BuyExecution.
- **Renewal workflow**: After the channel opens, follow the Coretime renewal guide to (a) configure the renewal pallet on SORA Polkadot, (b) fund the Coretime parachain account for periodic purchases, and (c) monitor renewal status via `coretimeAssignments` RPCs.
- **Monitoring**: Periodically query `hrmp.hrmpChannels([2025,<coretime_para_id>])` and the Coretime chain’s events to confirm execution capacity remains within headroom; schedule governance follow-ups if capacity increases are required.

**Phase 6: Sovereign DOT Move**

- **Withdraw & deposit**: From Root, send an XCM to the Relay withdrawing DOT from SORA’s sovereign relay account and depositing it into the Asset Hub sovereign location. Include Asset Hub `BuyExecution` and proof-size limits sized per Statemint 2.0.2 guidance.
- **Accounting**: After migration, verify the Relay sovereign account is near-zero while the Asset Hub sovereign account holds the DOT reserve used for reserve transfers.

**Phase 7: Compatibility & Apps**

- **Staking & deposits**: Re-test DOT staking flows that rely on Statemint 2.0.2 fixes (invulnerable deposit, XCM staking) to ensure SORA-origin messages succeed.
- **Wallets/UI**: Update explorers and wallets to fetch DOT balances from Asset Hub rather than Relay accounts; highlight Asset Hub runtime hash `0xe2af694d7e9f890e...` for monitoring.
- **Monitoring**: Track HRMP queues, XCM `VersionNotifiers`, and Statemint runtime upgrades so SORA Polkadot stays in sync with new releases.

**Phase 8: Timeline**

- **T0**: Runtime diff + try-runtime validated.
- **T0 + 1w**: Governance submission + communications.
- **T0 + 2w**: Runtime enactment, HRMP open, sovereign DOT move.
- **Post**: Audit + docs update; plan for next Statemint release if required.

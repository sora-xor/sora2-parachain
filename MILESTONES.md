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
- **Fast-track (optional)**: Use Council/Technical Committee to expedite if time-sensitive.
- **Rollout**: On enactment, validators/collators upgrade and begin producing with the new `spec_version`.

**Phase 5: HRMP to Asset Hub (Kusama)**

- **Channel open (one-sided)**: From SORA Kusama (para 2011), submit a single XCM to the Relay to request HRMP open to Asset Hub (para 1000). System parachains auto-accept; a single message is sufficient according to the Polkadot docs for para-to-system channels.
  - Parameters: `max_capacity`, `max_message_size` sized per traffic expectations.
  - Submission: Use `pallet_xcm::send` from Root origin to the Relay with `HrmpInitiateOpenChannel` (and, if needed by policy, `HrmpAcceptOpenChannel`).
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


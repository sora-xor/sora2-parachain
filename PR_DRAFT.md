Title: SORA Kusama: AHM readiness — trust KSM from Asset Hub, open HRMP to AH, and governance scripts

Summary
- Runtime: Trust KSM/DOT only from Asset Hub as reserve; stop trusting the Relay as reserve. Bump spec_version (Kusama build) to 15.
- HRMP: Add helper to build XCM Transact to open HRMP channel from SORA Kusama (2011) to Kusama Asset Hub (1000) and to estimate relay fees.
- Governance: Add script to submit preimage + propose referendum with 1 XOR deposit.
- Docs: Add AGENTS.md, MILESTONES.md, HRMP README, Chopsticks rehearsal guide + config.

Key Files
- runtime/src/xcm_config.rs — Adds `KsmFromAssetHub` ContainsPair and updates `IsReserve`.
- runtime/src/lib.rs — Bumps `spec_version` to 15.
- runtime/src/xcm_tests/tests.rs — Adds unit test `ksm_from_asset_hub_reserve_rule`.
- scripts/hrmp/open_to_asset_hub.ts — HRMP open builder + `--estimate` to compute relay fee.
- scripts/governance/submit_runtime_upgrade.ts — Preimage + proposal helper (1 XOR deposit).
- scripts/hrmp/README.md — Usage and estimation instructions.
- MILESTONES.md — Upgrade plan, explicit 1 XOR democracy deposit, HRMP fee paste-in lines.
- AGENTS.md — Repo and networks overview.
- chopsticks/ — Rehearsal config + guide.

Governance Parameters
- Democracy deposit: 1 XOR (1_000_000_000_000_000_000 plancks) per runtime config.
- Preimage deposits: base 1 planck, byte 1 planck (negligible).

HRMP Fee (KSM) — paste-in from live estimator
- Run:
  - `cd tools/js && node --loader ts-node/esm ./tmp/open_to_asset_hub.ts --relay-ws wss://kusama-rpc.polkadot.io --para 1000 --capacity 1000 --messageSize 1048576 --estimate`
- Paste here:
  - Relay HRMP fee (no tip): <PLANCKS> plancks (~<KSM> KSM)
- Set XCM `BuyExecution.fees` to ≥2× plancks above.

Post-merge Actions
1) Build Wasm for Kusama and submit governance (preimage + referendum) using `scripts/governance/submit_runtime_upgrade.ts`.
2) On enactment, open HRMP using the prebuilt XCM and the above fee; verify channel status.
3) Move sovereign KSM to Asset Hub via Root-origin XCM; monitor Balances events on Relay and Asset Hub.
4) Coordinate wallets/UIs to route via Asset Hub; monitor XCMP delivery.

Validation
- Unit test added for reserve rule.
- Rehearsal instructions via Chopsticks.


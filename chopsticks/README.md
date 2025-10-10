Chopsticks rehearsal for SORA Kusama AHM readiness

Goal
- Rehearse the SORA Kusama runtime upgrade and HRMP open to Asset Hub using forked chains.

Prereqs
- Install chopsticks (node-based tool) and have ws endpoints for Kusama + Kusama Asset Hub
- Build the new SORA Kusama runtime Wasm artifact

What this config does
- Forks Kusama and Asset Hub at chosen blocks, starts local endpoints
- Applies a parachain block stream for the SORA parachain id (2011)
- Allows you to submit the democracy preimage and proposal on the local SORA Kusama
- Allows verifying:
  - system.setCode enactment
  - pallet_xcm::send Transact to Parent for HRMP
  - XCMP delivery readiness and channel status

Files
- configs/sora-kusama-migration.json: example targets and placeholder provider URLs

Steps (outline)
1) Start forks:
   chopsticks start --config chopsticks/configs/sora-kusama-migration.json
2) Connect polkadot.js/apps to local SORA Kusama endpoint
3) Submit preimage (system.setCode with new Wasm) and democracy proposal
4) Force-enact in local environment if needed for rehearsal
5) Submit polkadotXcm.send(dest=Parent, xcm=Transact(hrmpInitOpenChannel(1000,...)))
6) Inspect events and HRMP channel state

Notes
- This is a rehearsal-only flow; use real governance on production.


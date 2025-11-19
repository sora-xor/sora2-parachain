HRMP Open Channel Helper

This script builds an XCM v3 message to open an HRMP channel from SORA Kusama (para 2011) to Kusama Asset Hub (para 1000) by executing `hrmp.initOpenChannel` on the Relay (Parent) via `Transact`.

Key points (security + compatibility):
- WithdrawAsset + BuyExecution: The XCM includes `WithdrawAsset` and `BuyExecution` before `Transact` so the Relay has fees to execute the call. Fees are withdrawn as KSM from SORA’s sovereign account on the Relay (`parents: 1, interior: Here`). Ensure this account holds sufficient KSM.
- Dynamic weight: `Transact.requireWeightAtMost` is derived from `paymentInfo(...).weight.refTime` (WeightV2) for the relay runtime to avoid `WeightNotEnough` on upgrades. You can tune with `--weight-multiplier`.
- Fee selection: By default the script multiplies the Relay fee estimate by `--fee-multiplier` (default `2.0`) and encodes that amount into `BuyExecution.fees`. You can override with `--fee-plancks`.

Usage (build payload only):
- Print a `polkadotXcm.send(dest, xcm)` extrinsic you can submit as a SORA democracy preimage (encoded with SORA metadata):

  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --sora-ws wss://kusama.sora.org \
    --para 1000 --capacity 1000 --messageSize 1048576 \
    [--fee-plancks 20000000000 | --fee-multiplier 2.0] \
    [--weight-multiplier 1.2]

Usage (submit on SORA, if permitted):

  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --sora-ws wss://sora-kusama.example \
    --para 1000 --capacity 1000 --messageSize 1048576 \
    [--fee-plancks 20000000000 | --fee-multiplier 2.0] \
    [--weight-multiplier 1.2] \
    --seed "//Alice" --submit

Dry-run fee estimate (no extrinsic built):

  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --para 1000 --capacity 1000 --messageSize 1048576 \
    --estimate

Message template (XCM v3):
- Destination: `Parent` (`{ V3: { parents: 1, interior: 'Here' } }`)
- Instructions:
  - `WithdrawAsset`: `[ { id: { Concrete: { parents: 1, interior: 'Here' } }, fun: { Fungible: <fee_plancks> } } ]`
  - `BuyExecution`: `{ fees: <same as above>, weightLimit: { Limited: { refTime: <derived>, proofSize: <derived> } } }`
  - `Transact`: `{ originKind: 'Native', requireWeightAtMost: { refTime: <derived>, proofSize: <derived> }, call: { encoded: <hrmp.initOpenChannel call> } }`

Operational notes:
- Ensure the SORA sovereign account on Kusama holds enough KSM to cover `BuyExecution.fees`.
- If Relay weight or fees change due to runtime upgrades, adjust `--fee-multiplier` or pass `--fee-plancks` explicitly. The script already derives weight to avoid hard-coded caps.

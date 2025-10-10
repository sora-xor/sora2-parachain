Opening HRMP channel from SORA Kusama (para 2011) to Kusama Asset Hub (para 1000)

- Prereqs:
  - Governance keys to dispatch Root-origin XCM on SORA Kusama
  - A relay chain account with sufficient fees for the HRMP open handling

- One-sided open (para-to-system):
  - System parachains (Asset Hub) auto-accept. You only need to send a single XCM from SORA Kusama to the Relay requesting the HRMP open to para 1000 with desired limits.

- Suggested parameters:
  - max_capacity: 1000
  - max_message_size: 1_048_576 (1 MiB)

- Polkadot.js API snippet (TypeScript):

  import { ApiPromise, WsProvider } from '@polkadot/api';
  import { hexAddPrefix } from '@polkadot/util';

  async function main() {
    const provider = new WsProvider('<ws://your-sora-kusama-node>');
    const api = await ApiPromise.create({ provider });

    // XCM v3 message to Relay to initiate HRMP open to Asset Hub (1000)
    const xcm = {
      V3: [
        { Transact: {
            originKind: 'Native',
            requireWeightAtMost: { refTime: 3_000_000_000, proofSize: 0 },
            call: {
              encoded: api.createType('Bytes',
                // runtime call: polkadotXcm.forceHrmpOpenChannel(dest_para=1000, maxCap, maxSize)
                // Alternatively, use hrmp.hrmpInitOpenChannel via call encoding on Relay.
                // Provide a properly encoded call for the Relay runtime you target.
                hexAddPrefix('0x00')
              )
            }
        }}
      ]
    };

    // Destination: Relay
    const dest = { V3: { parents: 1, interior: 'Here' } };

    // Send via pallet_xcm::send from Root
    const tx = api.tx.polkadotXcm.send(dest as any, xcm as any);
    // Sign with governance account via external signer
    console.log('Submit this extrinsic from Root through governance:', tx.toHex());
  }
  main().catch(console.error);

- Estimate relay KSM fees for HRMP open
- You can estimate the exact fee required on the relay for `hrmp.initOpenChannel` and set `BuyExecution.fees` accordingly:

  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --para 1000 --capacity 1000 --messageSize 1048576 \
    --estimate --relay-estimate-address <your-KSM-address>

  The script prints the fee in plancks and KSM. Set `BuyExecution.fees` to at least 2x that value.

- Alternative: use Polkadot-JS Apps
  - Go to SORA Kusama network, submit pallet_xcm -> send
  - dest: { parents: 1, interior: Here }
  - message: XCM V3 with a Transact containing the HRMP open call to para 1000
  - Ensure fees are covered via BuyExecution if required by your origin/weights

Verification
- Check HRMP channel status between 2011 and 1000 in the relay chain UI
- Confirm XcmpQueue events and successful lateral message delivery

Notes
- For para-to-system channels, a single message is sufficient (system para auto-accepts)
- Coordinate channel limits with expected traffic

Opening HRMP channel to Kusama Coretime (for auto-renewal)

- Coretime system parachain requires an HRMP channel from your parachain to deliver renewal messages.
- Identify the Coretime parachain ID on Kusama (consult Polkadot Dev Docs or chain state).
- Build and submit the same XCM Transact to Parent, replacing the `para` argument with the Coretime ID.

- Fee estimation (replace PARA with Coretime ID):

  cd tools/js
  node --loader ts-node/esm ./tmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --para <CORETIME_PARA_ID> --capacity 1000 --messageSize 1048576 \
    --estimate

- Set `BuyExecution.fees` in the XCM to ≥ 2× the printed plancks value to ensure execution.
- After the channel opens, configure the automatic renewal flow per https://docs.polkadot.com/develop/parachains/deployment/coretime-renewal/

/*
Construct an XCM to open an HRMP channel from SORA Kusama (para 2011)
to Kusama Asset Hub (para 1000) by sending a Transact to the Relay.

This script connects to the Relay to encode the hrmp.initOpenChannel call,
then builds a pallet_xcm::send extrinsic payload for SORA. You can either:
- Use the printed extrinsic hex as a democracy preimage on SORA, or
- Sign and submit directly if you control a Root-capable origin in a testnet.

Usage (build payload only):
  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --para 1000 --capacity 1000 --messageSize 1048576

Usage (submit on SORA, if permitted):
  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \
    --relay-ws wss://kusama-rpc.polkadot.io \
    --sora-ws wss://sora-kusama.example \
    --para 1000 --capacity 1000 --messageSize 1048576 \
    --seed "//Alice" --submit
*/

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

function arg(name: string, def?: string) {
  const i = process.argv.indexOf(name);
  if (i >= 0 && i + 1 < process.argv.length) return process.argv[i + 1];
  if (def !== undefined) return def;
  throw new Error('Missing ' + name);
}

async function main() {
  const relayWs = arg('--relay-ws');
  const soraWs = process.argv.includes('--submit') ? arg('--sora-ws') : undefined;
  const submit = process.argv.includes('--submit');
  const seed = submit ? arg('--seed') : undefined;
  const para = Number(arg('--para'));
  const capacity = Number(arg('--capacity', '1000'));
  const messageSize = Number(arg('--messageSize', '1048576'));
  const estimate = process.argv.includes('--estimate');
  const relayEstimateAddr = process.argv.includes('--relay-estimate-address')
    ? arg('--relay-estimate-address')
    : undefined;

  // 1) Build Relay call: hrmp.initOpenChannel(dest, capacity, messageSize)
  const relayApi = await ApiPromise.create({ provider: new WsProvider(relayWs) });
  const relayCall = relayApi.tx.hrmp.hrmpInitOpenChannel(para, capacity, messageSize);
  const relayCallBytes = relayCall.method.toU8a();
  console.log('Relay hrmp call hash:', relayCall.method.hash.toHex());

  if (estimate) {
    // Prefer rpc.payment.queryInfo to avoid requiring a signer address
    const info = await relayApi.rpc.payment.queryInfo(relayCall.toHex());
    const feePlancks = (info.partialFee as any).toBigInt ? (info.partialFee as any).toBigInt() : BigInt(info.partialFee.toString());
    const ksm = Number(feePlancks) / 1e12;
    console.log(`Estimated relay fee (no tip): ${info.partialFee.toString()} plancks (~${ksm} KSM)`);
    console.log('Recommendation: set BuyExecution.fees to at least 2x this estimate to be safe.');
  }

  // 2) Build XCM v3 Transact wrapping the relay call
  // Destination: Parent (Relay)
  const dest = { V3: { parents: 1, interior: 'Here' } } as any;
  const xcm = {
    V3: [
      { Transact: {
          originKind: 'Native',
          requireWeightAtMost: { refTime: 3_000_000_000, proofSize: 0 },
          call: { encoded: relayCallBytes }
      } }
    ]
  } as any;

  // 3) Build SORA extrinsic: pallet_xcm::send(dest, message)
  if (!submit) {
    // Build a SCALE-encoded call using a dummy API just to print extrinsic payload
    // Users should create preimage from: polkadotXcm.send(dest, xcm)
    const anyApi = await ApiPromise.create({ provider: new WsProvider(relayWs) });
    const call = (anyApi as any).tx.polkadotXcm.send(dest, xcm);
    console.log('Use this call for a SORA democracy preimage (polkadotXcm.send):');
    console.log(call.method.toHex());
    await anyApi.disconnect();
    await relayApi.disconnect();
    return;
  }

  const soraApi = await ApiPromise.create({ provider: new WsProvider(soraWs!) });
  const call = soraApi.tx.polkadotXcm.send(dest, xcm);
  const pair = new Keyring({ type: 'sr25519' }).addFromUri(seed!);
  await new Promise<void>((resolve, reject) => {
    call.signAndSend(pair, (res) => {
      if (res.status.isInBlock) console.log('In block', res.status.asInBlock.toHex());
      if (res.status.isFinalized) {
        console.log('Finalized', res.status.asFinalized.toHex());
        resolve();
      }
    }).catch(reject);
  });

  await soraApi.disconnect();
  await relayApi.disconnect();
}

main().catch((e) => { console.error(e); process.exit(1); });

/*
Build an XCM (v3) to open an HRMP channel from SORA Kusama (para 2011)
to Kusama Asset Hub (para 1000) by sending a Transact to the Relay.

Fixes per security review:
- Add WithdrawAsset + BuyExecution before Transact so execution is funded on the Relay.
- Encode a recommended fee into BuyExecution (derived from relay paymentInfo with multiplier).
- Derive Transact.requireWeightAtMost from paymentInfo.weight.refTime (with optional multiplier),
  rather than hard-coding a fixed limit which may cause WeightNotEnough.

This script connects to the Relay to encode hrmp.initOpenChannel and compute fee/weight,
then builds a pallet_xcm::send extrinsic payload for SORA. You can either:
- Use the printed extrinsic hex as a democracy preimage on SORA, or
- Sign and submit directly if you control a Root-capable origin in a testnet.

Usage (build payload only):
  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \\
    --relay-ws wss://kusama-rpc.polkadot.io \\
    --para 1000 --capacity 1000 --messageSize 1048576 \\
    [--fee-plancks 20000000000 | --fee-multiplier 2.0] \\
    [--weight-multiplier 1.2]

Usage (submit on SORA, if permitted):
  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/hrmp/open_to_asset_hub.ts \\
    --relay-ws wss://kusama-rpc.polkadot.io \\
    --sora-ws wss://sora-kusama.example \\
    --para 1000 --capacity 1000 --messageSize 1048576 \\
    [--fee-plancks 20000000000 | --fee-multiplier 2.0] \\
    [--weight-multiplier 1.2] \\
    --seed "//Alice" --submit

Notes:
- Fees are paid with KSM withdrawn from SORA’s sovereign account on the Relay (parents:1, interior:Here).
  Ensure that sovereign account holds enough KSM before sending.
- Use --estimate to print fee estimates from the Relay without building a call.
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
  const feePlancksArg = process.argv.includes('--fee-plancks') ? BigInt(arg('--fee-plancks')) : undefined;
  const feeMultiplier = Number(arg('--fee-multiplier', '2.0'));
  const weightMultiplier = Number(arg('--weight-multiplier', '1.2'));

  // 1) Build Relay call: hrmp.initOpenChannel(dest, capacity, messageSize)
  const relayApi = await ApiPromise.create({ provider: new WsProvider(relayWs) });
  const relayCall = relayApi.tx.hrmp.hrmpInitOpenChannel(para, capacity, messageSize);
  const relayCallBytes = relayCall.method.toU8a();
  console.log('Relay hrmp call hash:', relayCall.method.hash.toHex());

  // Get payment info to derive weight/fees
  const kr = new Keyring({ type: 'sr25519' });
  const signer = kr.addFromUri('//Alice');
  const info = await relayCall.paymentInfo(signer.address);
  const rawFeePlancks = (info.partialFee as any).toBigInt ? (info.partialFee as any).toBigInt() : BigInt(info.partialFee.toString());
  const estWeight = (info as any).weight || (info as any).weightV2 || (info as any).weightInfo;
  const refTime = estWeight?.refTime ? BigInt(estWeight.refTime.toString()) : BigInt(3_000_000_000);
  const proofSize = estWeight?.proofSize ? BigInt(estWeight.proofSize.toString()) : BigInt(0);

  if (estimate) {
    const ksm = Number(rawFeePlancks) / 1e12;
    console.log(`Estimated relay fee (no tip): ${rawFeePlancks.toString()} plancks (~${ksm} KSM)`);
    console.log(`Estimated weight: refTime=${refTime.toString()}, proofSize=${proofSize.toString()}`);
    console.log(`Recommendation: set BuyExecution.fees to at least ${feeMultiplier}x this estimate and weight multiplier ~${weightMultiplier}.`);
    await relayApi.disconnect();
    return;
  }

  // 2) Build XCM v3: WithdrawAsset -> BuyExecution -> Transact
  // Destination: Parent (Relay)
  const dest = { V3: { parents: 1, interior: 'Here' } } as any;

  // Compute fee and weight caps
  const recommendedFee = feePlancksArg ?? BigInt(Math.ceil(Number(rawFeePlancks) * feeMultiplier));
  const weightCap = {
    refTime: BigInt(Math.ceil(Number(refTime) * weightMultiplier)),
    proofSize: BigInt(Math.ceil(Number(proofSize) * weightMultiplier)),
  } as any;

  const feeAsset = {
    id: { Concrete: { parents: 1, interior: 'Here' } },
    fun: { Fungible: recommendedFee },
  } as any;

  const xcm = {
    V3: [
      { WithdrawAsset: [feeAsset] },
      { BuyExecution: { fees: feeAsset, weightLimit: { Limited: weightCap } } },
      { Transact: {
          originKind: 'Native',
          requireWeightAtMost: weightCap,
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

/*
Submit a runtime upgrade via Democracy on SORA Kusama.

Usage:
  TS_NODE_TRANSPILE_ONLY=1 ts-node scripts/governance/submit_runtime_upgrade.ts \
    --ws wss://sora-kusama.example \
    --seed "//Alice" \
    --wasm ./target/release/wbuild/sora2_parachain_runtime/sora2_parachain_runtime.compact.compressed.wasm \
    --deposit 1000000000000 \
    --mode preimage+propose

Modes:
  - preimage: submit preimage only
  - propose: propose using a preimage hash (requires --hash)
  - preimage+propose: do both in sequence
*/

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import fs from 'fs';

type Mode = 'preimage' | 'propose' | 'preimage+propose';

async function main() {
  const ws = getArg('--ws');
  const seed = getArg('--seed');
  const wasmPath = getArg('--wasm');
  const deposit = BigInt(getArg('--deposit', '0'));
  const mode = (getArg('--mode', 'preimage') as Mode);
  const preimageHashArg = getArg('--hash', '');

  const provider = new WsProvider(ws);
  const api = await ApiPromise.create({ provider });

  const pair = new Keyring({ type: 'sr25519' }).addFromUri(seed);

  let call;
  let callHash;
  if (mode !== 'propose') {
    if (!fs.existsSync(wasmPath)) throw new Error('WASM file not found: ' + wasmPath);
    const wasm = fs.readFileSync(wasmPath);
    call = api.tx.system.setCode(wasm);
    callHash = call.method.hash.toHex();
    console.log('Preimage call hash:', callHash);

    const note = api.tx.preimage.notePreimage(call.method.toU8a());
    const noteHash = await signAndSend(api, pair, note);
    console.log('Submitted preimage. tx hash:', noteHash);

    if (mode === 'preimage') {
      await api.disconnect();
      return;
    }
  }

  const hashToPropose = preimageHashArg || callHash;
  if (!hashToPropose) throw new Error('Missing --hash for propose mode');
  const propose = api.tx.democracy.propose(hashToPropose, deposit);
  const proposeHash = await signAndSend(api, pair, propose);
  console.log('Submitted democracy proposal. tx hash:', proposeHash);

  await api.disconnect();
}

async function signAndSend(api: ApiPromise, pair: any, tx: any) {
  return new Promise<string>((resolve, reject) => {
    tx.signAndSend(pair, (res: any) => {
      if (res.status.isInBlock) {
        console.log('In block', res.status.asInBlock.toHex());
      }
      if (res.status.isFinalized) {
        console.log('Finalized', res.status.asFinalized.toHex());
        resolve(res.txHash.toHex());
      }
    }).catch(reject);
  });
}

function getArg(name: string, def?: string): string {
  const i = process.argv.indexOf(name);
  if (i >= 0 && i + 1 < process.argv.length) return process.argv[i + 1];
  if (def !== undefined) return def;
  throw new Error('Missing ' + name);
}

main().catch((e) => { console.error(e); process.exit(1); });


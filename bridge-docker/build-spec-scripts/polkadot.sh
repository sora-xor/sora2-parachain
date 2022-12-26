
polkadot build-spec --chain rococo-local > rococo.json 

for para_id in 2000 2011
do
  # Add parachain config to chain spec
  jq \
    --rawfile genesis "$para_id-genesis" \
    --rawfile validation "$para_id-genesis-wasm" \
    --arg para_id "$para_id" \
    '.genesis.runtime.runtime_genesis_config.paras.paras += [[($para_id | tonumber), {
      "genesis_head": $genesis,
      "validation_code": $validation,
      "parachain": true
  }]]' rococo.json > rococo.json.tmp && mv rococo.json.tmp rococo.json
done

# Open HRMP channels
jq \
  '.genesis.runtime.runtime_genesis_config.hrmp.preopenHrmpChannels += [
    [2011, 2000, 8, 1048576],
    [2000, 2011, 8, 1048576]
]' rococo.json > rococo.json.tmp && mv rococo.json.tmp rococo.json

sed -i 's/1e+18/1000000000000000000/' rococo.json

polkadot build-spec --chain rococo.json --raw > rococo-raw.json
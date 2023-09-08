<img alt="SORA logo" src="https://static.tildacdn.com/tild3664-3939-4236-b762-306663333564/sora_small.svg"/>

# Overview

This is FRAME-based Substrate node of SORA2.
This repo contains code of node, pallets, runtime.

# Quick start

### Build 

For Kusama:
```sh
make kusama
```

For Polkadot:
```sh
make polkadot
```

For Rococo:
```sh
make rococo
```

### Test

For Kusama:
```sh
make test-kusama
```

For Polkadot:
```sh
make test-polkadot
```

For Rococo:
```sh
make test-rococo
```

# Rust Analyzer
Since project can only be compiled with features it would be convenient to set feature in rust analyzer

.vscode/settings.json may look like 

```json
{
    "rust-analyzer.cargo.features": [
        "rococo"
    ]
}
```
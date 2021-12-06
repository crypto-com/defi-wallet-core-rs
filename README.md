# DeFi Wallet Core

## Pre-requisites

https://rustup.rs

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cargo install uniffi_bindgen
brew install ktlint
brew install swiftformat
```

## Generate bindings

### Android
```bash
uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language kotlin --out-dir bindings/android
```

### iOS
```bash
uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language swift --out-dir bindings/ios
```

### WASM
```bash
wasm-pack build --scope crypto-com --out-dir ../bindings/wasm common
```

## Examples

### WASM
1. generate the bindings as above
2. `cd example/js-example`
3. `npm install`
4. `npm start`
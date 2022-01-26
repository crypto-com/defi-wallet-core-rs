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
wasm-pack build --scope crypto-com bindings/wasm
```

## Building

## Building Proto files from source

The single `proto-build` crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs), then

    cd proto-build # enter `proto-build`, so that we can clone repos and build proto files relative to it
    cargo run # build proto files and output to `../proto/src/prost` folder

or simply run `make proto`.

### Android
install android ndk: https://developer.android.com/ndk

suggest version r22 https://developer.android.google.cn/ndk/downloads/older_releases.html

set env NDK_HOME

```bash
make android
```

### iOS
```bash
make ios
```


## Examples

### WASM
1. generate the bindings as above
2. `cd example/js-example`
3. `npm install`
4. `npm start`

## Integration Tests
### WASM
#### Without nix
``` bash
make wasm-tests
```
#### With nix
``` bash
make wasm-ci-tests
```

### Python
``` bash
make python-tests
```

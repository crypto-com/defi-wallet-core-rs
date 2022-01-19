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
build in macos or linux

requires android sdk and ndk, java sdk

install android ndk: https://developer.android.com/ndk

suggest version r22 https://developer.android.google.cn/ndk/downloads/older_releases.html

android studio: https://developer.android.com/studio

set env ANDROID_SDK_ROOT,NDK_HOME,JAVA_HOME

such as

```bash
export ANDROID_SDK_ROOT={DIR}/Android/sdk
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/22.1.7171670
export JAVA_HOME={DIR}/openjdk/17.0.1_1/
```

build:
```bash
make android
```

### iOS

build in macos

requires xcode

build:
```bash
make ios
```

## Examples

### WASM
1. generate the bindings as above
2. `cd example/js-example`
3. `npm install`
4. `npm start`

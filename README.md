# DeFi Wallet Core

## Pre-requisites

- https://rustup.rs
- `uniffi_bindgen` 0.17.0 or newer

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cargo install uniffi_bindgen
uniffi-bindgen --version # check the version is 0.17.0 or newer
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

suggest version r22 https://github.com/android/ndk/wiki/Unsupported-Downloads

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

Requires Xcode 13+

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

## Integration Tests
### WASM
#### Without nix
Please install `pystarport`, `supervisord`, `chain-maind v3.3.3` or newer before running this test.

``` bash
make wasm-tests
```
#### With nix
Please install `nix` before running this test.

``` bash
make wasm-ci-tests
```

#### Cargo test
The recommended way to use `wasm-bindgen-test` is with `wasm-pack`, since it will handle installing the test runner, installing a WebDriver client for your browser, and informing cargo how to use the custom test runner. However, you can also manage those tasks yourself, if you wish.

It is also possible to use `cargo test` instead of `wasm-pack`

1. Install wasm-bindgen-cli
    ``` bash
    cargo install wasm-bindgen-cli
    ```

2. Install chromdriver (for chrome)
    ``` bash
    brew install --cask chromedriver
    ```

3. Run the test in binding/wasm folder
    ``` bash
    cargo test
    ```

##### References
- [Testing in headless browsers without wasm-pack](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html#appendix-testing-in-headless-browsers-without-wasm-pack)
- [Using wasm-bindgen-test without wasm-pack](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack)

### Python
#### With nix
Please install `nix` before running this test.

``` bash
make python-tests
```

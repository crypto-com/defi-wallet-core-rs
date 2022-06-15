# DeFi Wallet Core
[![Main Build Status][build-image]][build-link]
[![Audit Status][audit-image]][audit-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]

DeFi Wallet Core is an open-source cross-platform library that implements low-level cryptographic wallet functionality
for Cosmos SDK-based (such as Crypto.org Chain) and Ethereum-based (such as Cronos) blockchain networks.
It is leveraged in DeFi Wallet-related projects as well as in Cronos Play and other projects.
The codebase is primarily in Rust and provides cross-language bindings using multiple tools:

- [UniFFI] is used to generate bindings for both Kotlin (for Android apps) and Swift (for iOS apps); it could also be used to generate bindings for Python and Ruby.
- [wasm-bindgen] is used to generate bindings for JavaScript and TypeScript to facilitate interactions with the Wasm modules (for web browser extensions or other web-related apps).
- [CXX] is used to generate bindings for C++.

## Pre-requisites

Naturally, you will need the [Rust toolchain] installed.
Besides that, for [UniFFI] language bindings, you will need  `uniffi_bindgen` 0.18.0 or newer installed as well as corresponding language formatters:

```bash
cargo install uniffi_bindgen
uniffi-bindgen --version # check the version is 0.18.0 or newer
brew install ktlint
brew install swiftformat
```

For the JavaScript-Wasm bindings, you will need the `wasm-pack` to be installed:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Depending on your target platform, you may also need additional tooling for a given platform (e.g. Android NDK).

## Generate bindings

You can generate the language bindings for different platforms using the commands below.

### Android
```bash
uniffi-bindgen generate common/src/common.udl --config common/uniffi.toml --language kotlin --out-dir bindings/android
```

### iOS
```bash
uniffi-bindgen generate common/src/common.udl --config common/uniffi.toml --language swift --out-dir bindings/ios
```

### JavaScript-Wasm
```bash
wasm-pack build --scope crypto-com bindings/wasm
```

## Building

## Building Proto files from source

The single `proto-build` crate in this repo clones and rebuilds proto files for
all other crates, simply make the required edits in [main.rs](proto-build/main.rs), then

```bash
cd proto-build # enter `proto-build`, so that we can clone repos and build proto files relative to it
cargo run # build proto files and output to `../proto/src/prost` folder
```

or simply run `make proto`.

### Android
Building for Android is currently supported on macOS and Linux.

It requires Android SDK (e.g. via [Android Studio]), [Android NDK] ([the suggestion version is r22]), and Java SDK (e.g. OpenJDK) to be installed.

First, you need to set the required environment variables `ANDROID_SDK_ROOT`, `NDK_HOME`, and `JAVA_HOME`. For example:

```bash
export ANDROID_SDK_ROOT={DIR}/Android/sdk
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/22.1.7171670
export JAVA_HOME={DIR}/openjdk/17.0.1_1/
```

Then you can build the project as follows:

```bash
make android
```

### iOS

Building for iOS is currently supported on macOS.

It requires Xcode 13 or newer to be installed.

Then you can build the project as follows:
```bash
make ios
```

### C++
Build `bindings/cpp` and `example/cpp_example` as follows:
```bash
make build_cpp
```

Build `bindings/cpp` and `example/cpp_example`, and run the example programs. Please note that errors could happen if devnets (chainmain and cronos) are not running.
```bash
make cpp
```

## Examples
There are two sample programs for a web app as well as for a web browser extension.

### Wasm
This sample shows how the generated package can be used in JavaScript.

1. generate the bindings as above
2. `cd example/js-example`
3. `npm install`
4. `npm start`

### web browser extension example
This sample shows how the generated package can be used in a web browser extension.
1. make wasmweb
2. `cd example/extension-example`
3. `npm install`

## Unit and Integration Tests
There are several unit and integration tests for different target platforms. Here are the instructions how to run them.

### Wasm
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
### C++
``` bash
make cpp-ci-tests # build bindings and examples, then run the test
# or
make cpp-tests # run the test without building bindings and examples
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


## Documentation
### C++
``` bash
make cpp-docs
```
For a more detailed setup, check the [DeFi Wallet Core Cpp Documents Generation Guide](./docs/cpp/README.md)

[//]: # (badges)

[build-image]: https://github.com/crypto-com/defi-wallet-core-rs/actions/workflows/ci.yml/badge.svg
[build-link]: https://github.com/crypto-com/defi-wallet-core-rs/actions/workflows/ci.yml
[audit-image]: https://github.com/crypto-com/defi-wallet-core-rs/actions/workflows/audit.yml/badge.svg
[audit-link]: https://github.com/crypto-com/defi-wallet-core-rs/actions/workflows/audit.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/crypto-com/defi-wallet-core-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg

[//]: # (general links)

[UniFFI]: https://github.com/mozilla/uniffi-rs
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[CXX]: https://cxx.rs
[Rust toolchain]: https://rustup.rs
[Android NDK]: https://developer.android.com/ndk
[the suggestion version is r22]: https://github.com/android/ndk/wiki/Unsupported-Downloads
[Android Studio]: https://developer.android.com/studio
# ios example

## Pre-requisites


```
rustup target add aarch64-apple-ios x86_64-apple-ios
check Cargo.toml add crate-type = ["staticlib"..]
```

## Generate bindings
```bash
uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language swift --out-dir bindings/ios
```

## build
Compile according to the required platform:
```bash
cargo build --target aarch64-apple-ios --release
cargo build --target x86_64-apple-ios --release
```
## code setting

add libdefi_wallet_core_common.a to lib.ai/
add dwc_commonFFI.h to include/
add common.swift to $(PROJECT_DIR)/ios-example/


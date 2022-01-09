#!/bin/bash

rustup target add x86_64-apple-ios aarch64-apple-ios || exit 1
uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language swift --out-dir bindings/ios || exit 1
cargo build --target aarch64-apple-ios -p defi-wallet-core-common --release || exit 1
cargo build --target x86_64-apple-ios -p defi-wallet-core-common --release || exit 1
mkdir -p mobile_modules/ios_module/lib.a
lipo -create target/aarch64-apple-ios/release/libdefi_wallet_core_common.a target/x86_64-apple-ios/release/libdefi_wallet_core_common.a -output mobile_modules/ios_module/lib.a/libdefi_wallet_core_common.a || exit 1

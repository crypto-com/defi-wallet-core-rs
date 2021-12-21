#!/bin/bash

rustup target add x86_64-apple-ios aarch64-apple-ios || exit 1
uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language swift --out-dir bindings/ios || exit 1
cargo build --target aarch64-apple-ios --release || exit 1
cargo build --target x86_64-apple-ios --release || exit 1

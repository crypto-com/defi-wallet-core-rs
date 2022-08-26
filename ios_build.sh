#!/bin/bash
if [ "$1" == "x86" ]; then
        echo "Build x86 only"
fi

OS=`uname | tr 'A-Z' 'a-z'`
if [ "$OS" != "darwin" ]
then
        echo "not support for $OS"
fi

if [ "$1" != "x86" ]; then
        rustup target add x86_64-apple-ios || exit 1
fi
rustup target add x86_64-apple-ios || exit 1

uniffi-bindgen generate common/src/common.udl --config common/uniffi.toml --language swift --out-dir bindings/ios || exit 1
if [ "$1" != "x86" ]; then
        cargo build --features uniffi-binding --target aarch64-apple-ios -p defi-wallet-core-common --release || exit 1
fi
cargo build --features uniffi-binding --target x86_64-apple-ios -p defi-wallet-core-common --release || exit 1
mkdir -p mobile_modules/ios_module/dwclib/dwclib/lib.a
if [ "$1" != "x86" ]; then
        lipo -create target/aarch64-apple-ios/release/libdefi_wallet_core_common.a target/x86_64-apple-ios/release/libdefi_wallet_core_common.a -output mobile_modules/ios_module/dwclib/dwclib/lib.a/libdefi_wallet_core_common.a || exit 1
else
        cp target/x86_64-apple-ios/release/libdefi_wallet_core_common.a mobile_modules/ios_module/dwclib/dwclib/lib.a/libdefi_wallet_core_common.a
fi
mkdir -p mobile_modules/ios_module/dwclib/dwclib/include
cp bindings/ios/dwc_commonFFI.h mobile_modules/ios_module/dwclib/dwclib/include/ || exit 1
cp bindings/ios/common.swift mobile_modules/ios_module/dwclib/dwclib/ || exit 1
cd mobile_modules/ios_module/dwclib/
type strip || exit 1
# Because the build needs to be signed, so comment first
# xcodebuild SYMROOT="./build" -configuration Release -target dwclib -arch arm64 -sdk iphoneos build || exit 1
# strip build/Release-iphoneos/dwclib.framework/dwclib || exit 1
xcodebuild SYMROOT="./build" -configuration Release -target dwclib -arch x86_64 -sdk iphonesimulator build || exit 1
# Reduce package size
# strip build/Release-iphonesimulator/dwclib.framework/dwclib || exit 1
xcodebuild -scheme dwclib -destination 'platform=iOS Simulator,name=iPhone 12' test || exit 1
cd -

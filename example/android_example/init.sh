#!/bin/bash

mkdir -p app/libs
cp ../../NDK/libs/jna-min.jar app/libs/
mkdir -p app/src/main/java/com/defi/wallet/core/common
cp ../../bindings/android/com/defi/wallet/core/common/common.kt app/src/main/java/com/defi/wallet/core/common/ || exit 1
mkdir -p app/src/main/jniLibs/armeabi/
cp ../../target/aarch64-linux-android/release/libdefi_wallet_core_wasm.so app/src/main/jniLibs/armeabi/libdwc-common.so || exit 1
cp ../../NDK/libs/aarch64/libjnidispatch.so app/src/main/jniLibs/armeabi/ || exit 1
mkdir -p app/src/main/jniLibs/armeabi-v7a/
cp ../../target/armv7-linux-androideabi/release/libdefi_wallet_core_wasm.so app/src/main/jniLibs/armeabi-v7a/libdwc-common.so || exit 1
cp ../../NDK/libs/armv7/libjnidispatch.so app/src/main/jniLibs/armeabi-v7a/ || exit 1
mkdir -p app/src/main/jniLibs/x86/
cp ../../target/i686-linux-android/release/libdefi_wallet_core_wasm.so app/src/main/jniLibs/x86/libdwc-common.so || exit 1
cp ../../NDK/libs/x86/libjnidispatch.so app/src/main/jniLibs/x86/ || exit 1
cp ../../bindings/android/com/defi/wallet/core/common/common.kt app/src/main/java/com/defi/wallet/core/common/common.kt || exit 1

echo "finish"

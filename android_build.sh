#!/bin/bash

if [ "$1" == "x86" ]; then
        echo "Build x86 only"
fi

if [ ! -n "$NDK_HOME" ]; then
        echo "Env NDK_HOME is empty"
        exit 1
fi

NDK_VERSION=`cat $NDK_HOME/source.properties | grep Pkg.Revision | awk '{print $3}' | awk -F. '{print $1}'`
echo "NDK_VERSION is $NDK_VERSION"


if [ $NDK_VERSION -lt 23 ];then
        echo "Requires Android ndk version >= 23"
        exit 1
else
        RUSTFLAGS+=" -L`pwd`/env/android"
fi

API=24

export ANDROID_NDK_ROOT=$NDK_HOME

OS=`uname | tr 'A-Z' 'a-z'`
if [ "$OS" != "darwin" -a  "$OS" != "linux" ]
then
        echo "not support for $OS"
        exit 1
fi

if [ "$OS" == "darwin" ];then
	TOOLCHAIN=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64
fi
if [ "$OS" == "linux" ];then
	TOOLCHAIN=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64
fi

mkdir -p NDK/libs

if [ ! -f "NDK/libs/jna.aar" ]
then
        wget https://github.com/java-native-access/jna/raw/5.10.0/dist/jna.aar -P NDK/libs/ || exit 1
fi

uniffi-bindgen generate common/src/common.udl --config common/uniffi.toml --language kotlin --out-dir bindings/android || exit 1

if [ "$1" != "x86" ]; then
        rustup target add aarch64-linux-android armv7-linux-androideabi || exit 1
fi
rustup target add x86_64-linux-android || exit 1

if [ "$1" != "x86" ]; then
        PATH=$PATH:$TOOLCHAIN/bin \
	TARGET_CC=aarch64-linux-android$API-clang \
	CXX=aarch64-linux-android$API-clang++ \
	TARGET_AR=llvm-ar \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android$API-clang \
	RUSTFLAGS=$RUSTFLAGS cargo build --features uniffi-binding --target aarch64-linux-android -p defi-wallet-core-common --release || exit 1

        PATH=$PATH:$TOOLCHAIN/bin \
	TARGET_CC=armv7a-linux-androideabi$API-clang \
	CXX=armv7a-linux-androideabi$API-clang++ \
	TARGET_AR=llvm-ar \
	CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=armv7a-linux-androideabi$API-clang \
	RUSTFLAGS=$RUSTFLAGS cargo build --features uniffi-binding --target armv7-linux-androideabi -p defi-wallet-core-common --release || exit 1
fi

PATH=$PATH:$TOOLCHAIN/bin \
TARGET_CC=x86_64-linux-android$API-clang \
CXX=x86_64-linux-android$API-clang++ \
TARGET_AR=llvm-ar \
CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=x86_64-linux-android$API-clang \
RUSTFLAGS=$RUSTFLAGS cargo build --features uniffi-binding --target x86_64-linux-android -p defi-wallet-core-common --release || exit 1

type strip || exit 1
mkdir -p mobile_modules/android_module/dwclib/libs
cp NDK/libs/jna.aar mobile_modules/android_module/dwclib/libs/
if [ "$1" != "x86" ]; then
        mkdir -p mobile_modules/android_module/dwclib/src/main/jniLibs/arm64-v8a || exit 1
        cp target/aarch64-linux-android/release/libdefi_wallet_core_common.so mobile_modules/android_module/dwclib/src/main/jniLibs/arm64-v8a/libdwc-common.so || exit 1
        strip mobile_modules/android_module/dwclib/src/main/jniLibs/arm64-v8a/libdwc-common.so
        mkdir -p mobile_modules/android_module/dwclib/src/main/jniLibs/armeabi-v7a || exit 1
        cp target/armv7-linux-androideabi/release/libdefi_wallet_core_common.so mobile_modules/android_module/dwclib/src/main/jniLibs/armeabi-v7a/libdwc-common.so || exit 1
        strip mobile_modules/android_module/dwclib/src/main/jniLibs/armeabi-v7a/libdwc-common.so
fi
mkdir -p mobile_modules/android_module/dwclib/src/main/jniLibs/x86_64 || exit 1
cp target/x86_64-linux-android/release/libdefi_wallet_core_common.so mobile_modules/android_module/dwclib/src/main/jniLibs/x86_64/libdwc-common.so || exit 1
strip mobile_modules/android_module/dwclib/src/main/jniLibs/x86_64/libdwc-common.so
mkdir -p mobile_modules/android_module/dwclib/src/main/java/com/defi/wallet/core/common || exit 1
cp bindings/android/com/defi/wallet/core/common/common.kt mobile_modules/android_module/dwclib/src/main/java/com/defi/wallet/core/common/ || exit 1

cd mobile_modules/android_module || exit 1
./gradlew build || exit 1
./gradlew dwclib:connectedAndroidTest || exit 1
cd -
cp mobile_modules/android_module/dwclib/build/outputs/aar/dwclib-release.aar NDK/libs/dwclib.aar || exit 1
mkdir -p example/android_example/app/libs
cp NDK/libs/dwclib.aar example/android_example/app/libs/
cp NDK/libs/jna.aar example/android_example/app/libs/

echo "finish"

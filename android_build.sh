#!/bin/bash

if [ ! -n "$NDK_HOME" ]; then
        echo "Env NDK_HOME is empty"
        exit 1
fi

mkdir -p NDK/libs
if [ ! -f "NDK/libs/jna-min.jar" ]
then
        wget https://github.com/java-native-access/jna/raw/5.10.0/dist/jna-min.jar -P NDK/libs/ || exit 1
fi
if [ ! -f "NDK/libs/aarch64/libjnidispatch.so" ]
then
        wget https://github.com/java-native-access/jna/raw/5.10.0/dist/android-aarch64.jar -P NDK/libs/aarch64/ || exit 1
        unzip -o NDK/libs/aarch64/android-aarch64.jar -d NDK/libs/aarch64/ || exit 1
        rm -f NDK/libs/aarch64/android-aarch64.jar
fi
if [ ! -f "NDK/libs/armv7/libjnidispatch.so" ]
then
        wget https://github.com/java-native-access/jna/raw/5.10.0/dist/android-armv7.jar -P NDK/libs/armv7/ || exit 1
        unzip -o NDK/libs/armv7/android-armv7.jar -d NDK/libs/armv7/ || exit 1
        rm -f NDK/libs/armv7/android-armv7.jar
fi
if [ ! -f "NDK/libs/x86/libjnidispatch.so" ]
then
        wget https://github.com/java-native-access/jna/raw/5.10.0/dist/android-x86.jar -P NDK/libs/x86/ || exit 1
        unzip -o NDK/libs/x86/android-x86.jar -d NDK/libs/x86/ || exit 1
        rm -f NDK/libs/x86/android-x86.jar
fi


MAKETOOL="$NDK_HOME/build/tools/make_standalone_toolchain.py"
#echo $MAKETOOL

if [ ! -x "$MAKETOOL" ]
then
        echo "Android NDK is not installed."
        exit 1
fi

uniffi-bindgen generate common/src/common.udl --config-path common/uniffi.toml --language kotlin --out-dir bindings/android || exit 1

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android || exit 1

if [ ! -d "NDK/arm64" ]
then
        "$MAKETOOL" --api 28 --arch arm64 --install-dir NDK/arm64 2> /dev/null || exit 1
else
        echo "arm64 ndk installed."
fi

if [ ! -d "NDK/arm" ]
then
        "$MAKETOOL" --api 28 --arch arm --install-dir NDK/arm 2> /dev/null || exit 1
else
        echo "arm ndk installed."
fi

if [ ! -d "NDK/x86" ]
then
        "$MAKETOOL" --api 28 --arch x86 --install-dir NDK/x86 2> /dev/null || exit 1
else
        echo "x86 ndk installed."
fi

cargo build --target aarch64-linux-android --release || exit 1
cargo build --target armv7-linux-androideabi --release || exit 1
cargo build --target i686-linux-android --release || exit 1


export NDK_VERSION=25.1.8937393
export API=21
export NDK_HOME=$HOME/Library/Android/sdk/ndk/$NDK_VERSION
export TARGET=aarch64-linux-android
export TOOLCHAIN=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64
export TARGET_CC=$TOOLCHAIN/bin/$TARGET$API-clang
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++ 
export TARGET_AR=$TOOLCHAIN/bin/llvm-ar 
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$TOOLCHAIN/bin/$TARGET$API-clang 
export RUSTFLAGS+=" -L`pwd`/../../env/android"
cargo build --target=$TARGET --release


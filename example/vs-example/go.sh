# run command in git shell of windows 
# or run in msys2 shell
export DST=$PWD/vs-example
cargo build --package defi-wallet-core-cpp --release
cp ../cpp-example/main.cc $DST
cp ../cpp-example/cronos.cc $DST
cp $(find ../../target/release -name "libcxxbridge1.a") $DST
cp ../../target/release/defi_wallet_core_cpp.* $DST
cp ../../target/cxxbridge/rust/cxx.h $DST
cp ../../target/cxxbridge/defi-wallet-core-cpp/src/*.h $DST
cp ../../target/cxxbridge/defi-wallet-core-cpp/src/*.cc $DST

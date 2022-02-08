cd ..\..\bindings\cpp 
cargo build --release
cd ..\..\example\vs-example
copy ..\..\target\release\defi_wallet_core_cpp.*  .\vs-example\
copy ..\..\bindings\cpp\bindings.h .\vs-example\
# run command in git shell of windows
# or run in msys2 shell
cargo build --package defi-wallet-core-cpp --release
cd ../cpp-example
python3 helper.py  --target_dir ../../target/release


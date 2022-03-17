# run command in git shell of windows
# or run in msys2 shell
cargo build --package defi-wallet-core-cpp --release
cp ../cpp-example
python3 helper.py

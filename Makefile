
cpp_example = ./example/cpp-example

.PHONY: wasm android ios test clean cleanall

wasm:
	wasm-pack build --scope crypto-com bindings/wasm

android:
	./android_build.sh

ios:
	./ios_build.sh

test:
	cargo test

clean:
	rm -rf target bindings/android bindings/ios

cleanall:
	rm -rf target bindings/android bindings/ios
	rm -rf NDK

mac_install:
	cargo install uniffi_bindgen
	brew install ktlint
	brew install swiftformat

cpp:
	cargo build --release
	cargo build
	cp ./target/release/libdefi_wallet_core_cpp.a $(cpp_example)
	cp ./target/cxxbridge/rust/cxx.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.cc $(cpp_example)
	cd $(cpp_example) && make


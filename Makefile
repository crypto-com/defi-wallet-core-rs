
cpp_example = ./example/cpp-example

.PHONY: wasm android ios test clean cleanall mac_install cpp python-tests lint-fix lint-py wasm-tests wasm-ci-tests proto

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
	./clean.sh

cleanall:
	rm -rf target bindings/android bindings/ios
	rm -rf NDK
	./clean.sh


mac_install:
	cargo install uniffi_bindgen
	brew install ktlint
	brew install swiftformat

cpp:
	cargo build --release
	cp ./target/release/libdefi_wallet_core_cpp.a $(cpp_example)
	cp ./target/cxxbridge/rust/cxx.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.cc $(cpp_example)
	cd $(cpp_example) && make

cppx86_64:
	cargo build --release --target x86_64-apple-darwin
	cp ./target/x86_64-apple-darwin/release/libdefi_wallet_core_cpp.a $(cpp_example)
	cp ./target/cxxbridge/rust/cxx.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.h $(cpp_example)
	cp ./target/cxxbridge/defi-wallet-core-cpp/src/*.cc $(cpp_example)
	cd $(cpp_example) && make x86_64


proto:
	cd proto-build && cargo run

python-tests:
	@nix-shell ./integration_tests/shell.nix --run scripts/python-tests

wasm-ci-tests:
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl start"
	sleep 10
	cd bindings/wasm/ && wasm-pack test --chrome --headless
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl stop"
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl clear"

wasm-tests:
	./scripts/wasm-tests

lint-py:
	flake8 --show-source --count --statistics \
          --format="::error file=%(path)s,line=%(row)d,col=%(col)d::%(path)s:%(row)d:%(col)d: %(code)s %(text)s" \

lint-nix:
	find . -name "*.nix" ! -path './example/*' | xargs nixpkgs-fmt --check

UNAME := $(shell uname)

cpp_example = ./example/cpp-example

.PHONY: wasm wasmweb android ios test clean cleanall mac_install cpp python-tests lint-fix lint-py wasm-tests wasm-ci-tests proto cpp-ci-tests cpp-tests mobile-release

wasm:
	wasm-pack build --scope crypto-com bindings/wasm

wasmweb:
	wasm-pack build -d target/wasmweb --scope crypto-com bindings/wasm --target web

android:
	./android_build.sh

ios:
	./ios_build.sh

test:
	cargo test --all-features

clean:
	rm -rf target bindings/android bindings/ios
	./clean.sh

cleanall:
	rm -rf target bindings/android bindings/ios
	rm -rf NDK
	./clean.sh

mobile-release:
	cp mobile_modules/android_module/dwclib/build/outputs/aar/dwclib-release.aar target/release/
	zip -q -r target/release/dwclib-framework-iphoneos.zip mobile_modules/ios_module/dwclib/build/Release-iphoneos/dwclib.framework
	zip -q -r target/release/dwclib-framework-iphonesimulator.zip mobile_modules/ios_module/dwclib/build/Release-iphonesimulator/dwclib.framework

mac_install:
	cargo install uniffi_bindgen
	brew install ktlint
	brew install swiftformat


build_cpp:
	cargo build --package defi-wallet-core-cpp --release
	cd $(cpp_example) && make build

cpp: build_cpp
	. ./scripts/.env && cd $(cpp_example) && make run

cppx86_64:
	rustup target add x86_64-apple-darwin
	cargo build --package defi-wallet-core-cpp --release --target x86_64-apple-darwin
	cd $(cpp_example) && make x86_64_build


proto:
	cd proto-build && cargo run

python-tests:
	@nix-shell ./integration_tests/shell.nix --run scripts/python-tests

wasm-ci-tests:
	export WASM_BINDGEN_TEST_TIMEOUT=60
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl start"
	cd bindings/wasm/ && wasm-pack test --chrome --headless && cd ../..
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl stop"
	@nix-shell ./integration_tests/shell.nix --run "scripts/chainmain-ctl clear"
	@nix-shell ./integration_tests/shell.nix --run "scripts/start-all"
	cd bindings/wasm/ && wasm-pack test --chrome --headless -- --features ibc-test --test ibc && cd ../..
	@nix-shell ./integration_tests/shell.nix --run "scripts/stop-all"

# No ibc test
wasm-tests:
	sh ./scripts/wasm-tests

# Full test with ibc cases
full-wasm-tests:
	sh ./scripts/full-wasm-tests

cpp-ci-tests: build_cpp
	make cpp-tests

cpp-tests: python-tests

# TODO or use other docs engine
cpp-docs: build_cpp
	grep -h -R -E "//" -A 1 -R --include "$(cpp_example)/defi-wallet-core-cpp/src/*.h" > cpp_docs.md
# add break line
	sed -i '' 's/--/---\n/g' cpp_docs.md
# add more spaces in beginning of line
	sed -i '' 's/  /    /g' cpp_docs.md
	sed -i '' "s/^\/\//    \/\//g" cpp_docs.md
	sed -i '' "s/^::/    ::/g" cpp_docs.md
# remove #endif lines
	sed -i '' 's/^#endif.*//g' cpp_docs.md
# remove } // lines
	sed -i '' 's/^} \/\/.*//g' cpp_docs.md


lint-py:
	flake8 --show-source --count --statistics \
          --format="::error file=%(path)s,line=%(row)d,col=%(col)d::%(path)s:%(row)d:%(col)d: %(code)s %(text)s" \

lint-nix:
	find . -name "*.nix" ! -path './example/*' | xargs nixpkgs-fmt --check

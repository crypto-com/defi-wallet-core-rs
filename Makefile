UNAME := $(shell uname)

cpp_example = ./example/cpp-example

cpp_docs = ./docs/cpp
.PHONY: wasm js wasmweb android ios test clean cleanall mac_install cpp python-tests lint-fix lint-py wasm-tests wasm-ci-tests proto cpp-ci-tests cpp-tests mobile-release

wasm:
	wasm-pack build --scope crypto-com bindings/wasm

js:
	wasm-pack build --scope crypto-com bindings/wasm
	./js_build.sh

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
	cd bindings/wasm/ && wasm-pack test --chrome --headless -- --features cronos-test,ibc-test --test ibc --test ethereum && cd ../..
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

# Choose the defualt cpp docs engine
cpp-docs: cpp-docs-gitbook

cpp-docs-doxygen: build_cpp
	@nix-shell ./docs/cpp/shell.nix --run "cd $(cpp_docs) && doxygen"
ifeq ($(UNAME), Darwin)
	open $(cpp_docs)/doxygen/html/index.html
endif

cpp-docs-sphinx: build_cpp
	@nix-shell ./docs/cpp/shell.nix --run "cd $(cpp_docs) && doxygen && cd sphinx && make html"
ifeq ($(UNAME), Darwin)
	open $(cpp_docs)/sphinx/_build/html/index.html
endif

cpp-docs-gitbook: build_cpp
	@nix-shell ./docs/cpp/shell.nix --run "\
	cd $(cpp_docs) && doxygen && doxybook2 \
		--input doxygen/xml \
		--output gitbook/src \
		--config config.json \
		--summary-input SUMMARY.md.tmpl \
		--summary-output gitbook/src/SUMMARY.md \
		&& cd gitbook/src && gitbook serve"


cpp-docs-mdbook: build_cpp
	@nix-shell ./docs/cpp/shell.nix --run "\
		cd $(cpp_docs) && doxygen && doxybook2 \
		--input doxygen/xml \
		--output mdbook/src \
		--config config.json \
		--summary-input SUMMARY.md.tmpl \
		--summary-output mdbook/src/SUMMARY.md \
		&& cd mdbook && mdbook serve --open"

lint-py:
	flake8 --show-source --count --statistics \
          --format="::error file=%(path)s,line=%(row)d,col=%(col)d::%(path)s:%(row)d:%(col)d: %(code)s %(text)s" \

lint-nix:
	find . -name "*.nix" ! -path './example/*' | xargs nixpkgs-fmt --check

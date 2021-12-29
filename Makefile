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
	./clean.sh

cleanall:
	rm -rf target bindings/android bindings/ios
	rm -rf NDK
	./clean.sh

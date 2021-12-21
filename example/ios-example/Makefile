.PHONY: x86 aarch64 clean

x86:
	cp ../../bindings/ios/dwc_commonFFI.h ios-example/include
	cp ../../bindings/ios/common.swift ios-example/
	mkdir -p ios-example/lib.a/
	cp ../../target/x86_64-apple-ios/release/libdefi_wallet_core_common.a ios-example/lib.a/

aarch64:
	cp ../../bindings/ios/dwc_commonFFI.h ios-example/include
	cp ../../bindings/ios/common.swift ios-example/
	mkdir -p ios-example/lib.a/
	cp ../../target/aarch64-apple-ios/release/libdefi_wallet_core_common.a ios-example/lib.a/

clean:
	rm -rf ios-example/include/dwc_commonFFI.h
	rm -rf ios-example/common.swift
	rm -rf ios-example/lib.a/*
	
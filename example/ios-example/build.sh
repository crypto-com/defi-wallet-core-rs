#!/bin/bash

case $1 in
	x86)
		cp ../../bindings/ios/dwc_commonFFI.h ios-example/include || exit 1
		cp ../../bindings/ios/common.swift ios-example/ || exit 1
		mkdir -p ios-example/lib.a/ || exit 1
		cp ../../target/x86_64-apple-ios/release/libdefi_wallet_core_common.a ios-example/lib.a/ || exit 1
		xcodebuild -configuration Debug -target ios-example -arch x86 -sdk `xcodebuild -showsdks | grep 'iphonesimulator' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	aarch64)
		cp ../../bindings/ios/dwc_commonFFI.h ios-example/include || exit 1
		cp ../../bindings/ios/common.swift ios-example/ || exit 1
		mkdir -p ios-example/lib.a/ || exit 1
		cp ../../target/aarch64-apple-ios/release/libdefi_wallet_core_common.a ios-example/lib.a/ || exit 1
		xcodebuild -configuration Debug -target ios-example -arch arm64 -sdk `xcodebuild -showsdks | grep 'iphoneos' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	*)
		echo "$1 is not supported"
esac

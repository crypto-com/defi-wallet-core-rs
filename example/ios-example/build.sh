#!/bin/bash

cp ../../bindings/ios/dwc_commonFFI.h ios-example/include || exit 1
cp ../../bindings/ios/common.swift ios-example/ || exit 1
mkdir -p ios-example/lib.a/ || exit 1
cp ../../mobile_modules/ios_module/lib.a/libdefi_wallet_core_common.a ./ios-example/lib.a || exit 1

case $1 in
	x86)
		xcodebuild SYMROOT="./build" -configuration Debug -target ios-example -arch x86 -sdk `xcodebuild -showsdks | grep 'iphonesimulator' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	arm64)
		xcodebuild SYMROOT="./build" -configuration Debug -target ios-example -arch arm64 -sdk `xcodebuild -showsdks | grep 'iphoneos' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	*)
		echo "$1 is not supported"
esac


#!/bin/bash

case $1 in
	x86_64)
		rm -rf dwclib.framework
		cp -r ../../mobile_modules/ios_module/dwclib/build/Release-iphonesimulator/dwclib.framework ./ || exit 1
		xcodebuild SYMROOT="./build" -configuration Debug -target ios-example -arch x86_64 -sdk `xcodebuild -showsdks | grep 'iphonesimulator' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	arm64)
		rm -rf dwclib.framework
		cp -r ../../mobile_modules/ios_module/dwclib/build/Release-iphoneos/dwclib.framework ./ || exit 1
		xcodebuild SYMROOT="./build" -configuration Debug -target ios-example -arch arm64 -sdk `xcodebuild -showsdks | grep 'iphoneos' | awk 'BEGIN{FS="-sdk"} {print $2}'` || exit 1
	;;
	*)
		echo "$1 is not supported"
esac


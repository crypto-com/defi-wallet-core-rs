#!/bin/bash
cd mobile_modules/android_module || exit 1
./gradlew clean || exit 1
cd -

cd example/android_example || exit 1
make clean
cd -

cd mobile_modules/ios_module/dwclib  || exit 1 
xcodebuild SYMROOT="./build" clean
cd -

rm -f mobile_modules/ios_module/lib.a/* || exit 1 
cd example/ios-example || exit 1
make clean
cd -

rm -rf bindings/wasm/target
rm -rf bindings/wasm/pkg/*
rm -rf example/extension-example/node_modules
rm -rf example/js-example/node_modules

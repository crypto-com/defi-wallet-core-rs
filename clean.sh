#!/bin/bash
cd mobile_modules/android_module || exit 1
./gradlew clean || exit 1
cd -

cd example/android_example || exit 1
make clean
cd -


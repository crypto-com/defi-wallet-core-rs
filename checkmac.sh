# to fix link error on macosx
# cd example/cpp-example
# otool -l libcxxbridge1.a > out
# otool -l libdefi_wallet_core_cpp.dylib > out
# check LC_BUILD_VERSION/minos is 10.15
if [[ $(uname) == "Darwin" ]]; then
	export MACOSX_DEPLOYMENT_TARGET=10.15
	echo "MACOSX_DEPLOYMENT_TARGET="$MACOSX_DEPLOYMENT_TARGET
fi

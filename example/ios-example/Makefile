.PHONY: x86_64 arm64 clean

x86_64:
	./build.sh x86_64
arm64:
	./build.sh arm64
clean:
	rm -rf dwclib.framework
	xcodebuild SYMROOT="./build" clean	

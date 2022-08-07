UNAME := $(shell uname)
sdk_dir = ./sdk


ifeq ($(shell uname -m), x86_64)
	CXX = g++
	TARGET_DIR = ../../target/release
endif

ifeq ($(shell uname -m), arm64)
	CXX = arch -x86_64 g++
	TARGET_DIR = ../../target/x86_64-apple-darwin/release
else

endif

ifeq ($(UNAME), Darwin)
	FLAGS=-framework Security -framework CoreFoundation
endif

ifeq ($(UNAME), Linux)
	FLAGS=-lpthread -ldl
endif

all: clean build
build: prepare static dynamic cmake
run: run_static run_dynamic

prepare:
	python3 helper.py --target_dir  $(TARGET_DIR)

libdefi_wallet_core_cpp.dylib:
ifeq ($(UNAME), Darwin)
	install_name_tool -id @rpath/libdefi_wallet_core_cpp.dylib $(sdk_dir)/lib/libdefi_wallet_core_cpp.dylib
endif

static:
	$(CXX) -o cppexamplestatic \
		main.cc \
		chainmain.cc \
		cronos.cc \
		wallet.cc \
		$(sdk_dir)/lib/libdefi_wallet_core_cpp.a \
		-std=c++11 \
		$(FLAGS)

dynamic: libdefi_wallet_core_cpp.dylib
ifeq ($(UNAME), Linux)
	$(CXX) -o cppexample \
		main.cc \
		chainmain.cc \
		wallet.cc \
		cronos.cc \
		$(sdk_dir)/include/*.cc \
		$(sdk_dir)/include/defi-wallet-core-cpp/src/*.cc \
		$(sdk_dir)/lib/libcxxbridge1.a \
		-std=c++11 \
		-L$(sdk_dir)/lib -Wl,-rpath=$(sdk_dir)/lib -Wall \
		-ldefi_wallet_core_cpp \
		$(FLAGS)
endif
ifeq ($(UNAME), Darwin)
	$(CXX) -o cppexample \
		main.cc \
		chainmain.cc \
		wallet.cc \
		cronos.cc \
		$(sdk_dir)/include/*.cc \
		$(sdk_dir)/include/defi-wallet-core-cpp/src/*.cc \
		$(sdk_dir)/lib/libcxxbridge1.a \
		-std=c++11 \
		$(sdk_dir)/lib/libdefi_wallet_core_cpp.dylib \
		-rpath $(sdk_dir)/lib \
		$(FLAGS)
endif

cmake:
	mkdir -p build
	cd build && cmake .. && make

run_static:
	./cppexamplestatic && ./build/cppexamplestatic

run_dynamic:
	export LD_LIBRARY_PATH=$(PWD) && ./cppexample && ./build/cppexample

clean:
	rm -f cppexample cppexamplestatic
	rm -rf build
	rm -rf $(sdk_dir)/lib
	rm -rf $(sdk_dir)/include

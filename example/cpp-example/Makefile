UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	FLAGS=-framework Security -framework CoreFoundation
endif

ifeq ($(UNAME), Linux)
	FLAGS=-lpthread -lssl -lcrypto -ldl
endif

all: build run
build: prepare static dynamic
run: run_static run_dynamic

prepare:
	python3 helper.py --target_dir ../../target/release

prepare_x86_64:
	python3 helper.py --target_dir ../../target/x86_64-apple-darwin/release

static:
	g++ -o cppexamplestatic \
		main.cc \
		chainmain.cc \
		cronos.cc \
		lib/libdefi_wallet_core_cpp.a \
		-std=c++11 \
		$(FLAGS)

dynamic:
	g++ -o cppexample \
		main.cc \
		chainmain.cc \
		cronos.cc \
		include/*.cc \
		include/defi-wallet-core-cpp/src/*.cc \
		lib/libcxxbridge1.a \
		-ldefi_wallet_core_cpp \
		-std=c++11 \
		$(FLAGS)  \
		-L lib \

x86_64_static:
	arch -x86_64 g++ -o cppexamplestatic \
		main.cc \
		chainmain.cc \
		cronos.cc \
		lib/libdefi_wallet_core_cpp.a \
		-std=c++11 \
		$(FLAGS)

x86_64_dynamic:
	arch -x86_64 g++ -o cppexample \
		main.cc \
		chainmain.cc \
		cronos.cc \
		include/*.cc \
		include/defi-wallet-core-cpp/src/*.cc \
		lib/libcxxbridge1.a \
		-ldefi_wallet_core_cpp \
		-std=c++11 \
		$(FLAGS)  \
		-L lib \

x86_64_build: prepare_x86_64 x86_64_static x86_64_dynamic

run_static:
	./cppexamplestatic

run_dynamic:
	export LD_LIBRARY_PATH=$(PWD) && ./cppexample

UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	FLAGS=-framework Security -framework CoreFoundation
endif

ifeq ($(UNAME), Linux)
	FLAGS=-lpthread -lssl -lcrypto -ldl
endif

all: build run
build: static dynamic
run: run_static run_dynamic

static:
	g++ -o cppexamplestatic main.cc cronos.cc libdefi_wallet_core_cpp.a -std=c++11 $(FLAGS)

dynamic:
	g++ -o cppexample main.cc cronos.cc defi-wallet-core-cpp/src/*.cc libcxxbridge1.a -ldefi_wallet_core_cpp -std=c++11 $(FLAGS)  -L .

x86_64: x86_64_build
	arch -x86_64 ./cppexample

x86_64_build:
	arch -x86_64 g++ -o cppexample main.cc cronos.cc libdefi_wallet_core_cpp.a -std=c++11 $(FLAGS)

run_static:
	./cppexamplestatic

run_dynamic:
	export LD_LIBRARY_PATH=$(PWD) && ./cppexample

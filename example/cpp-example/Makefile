UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
FLAGS=-framework Security -framework CoreFoundation
endif

all:
	g++ -o cppexample main.cc libdefi_wallet_core_cpp.a   -std=c++11 $(FLAGS)
	./cppexample

x86_64:
	arch -x86_64 g++ -o cppexample main.cc libdefi_wallet_core_cpp.a   -std=c++11 $(FLAGS)
	arch -x86_64 ./cppexample
	

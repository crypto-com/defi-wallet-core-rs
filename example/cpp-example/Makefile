UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
FLAGS=-framework Security -framework CoreFoundation
endif

all: static dynamic 
static:
	g++ -o cppexamplestatic  main.cc  libdefi_wallet_core_cpp.a  -std=c++11 $(FLAGS) 
	./cppexamplestatic

dynamic:
	g++ -o cppexample main.cc lib.rs.cc libcxxbridge1.a  -ldefi_wallet_core_cpp  -std=c++11 $(FLAGS)  -L .
	export LD_LIBRARY_PATH=$(PWD) && ./cppexample

x86_64:
	arch -x86_64 g++ -o cppexample main.cc libdefi_wallet_core_cpp.a   -std=c++11 $(FLAGS)
	arch -x86_64 ./cppexample
	

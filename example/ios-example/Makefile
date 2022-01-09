.PHONY: x86 arm64 clean

x86:
	./build.sh x86
arm64:
	./build.sh arm64
clean:
	rm -rf ios-example/include/dwc_commonFFI.h
	rm -rf ios-example/common.swift
	rm -rf ios-example/lib.a/*
	xcodebuild SYMROOT="./build" clean	

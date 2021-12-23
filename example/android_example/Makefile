.PHONY: app init clean

app:
	./gradlew build

init:
	./init.sh

clean:
	rm -rf app/libs/*
	rm -rf app/src/main/jniLibs/armeabi/*
	rm -rf app/src/main/jniLibs/armeabi-v7a/*
	rm -rf app/src/main/jniLibs/x86/*
	rm -rf build
	rm -rf app/build


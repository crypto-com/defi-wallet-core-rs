# compile
- kotlin source can be compilied in any android project.
- copy the SecureStorage.kt file to your project.
- path would be `yourproject/app/src/main/java/com/cronos/play/SecureStorage.kt`


# packaging jar file
for unreal engine, we need to package the kotlin class file into a jar file.
```
./gradlew assembleRelease
jar cvf SecureStorage.jar ./app/build/tmp/kotlin-classes/release/com/cronos/play/SecureStorage.class
```
#include "rust/cxx.h"
#include <jni.h>
namespace org {
namespace defi_wallet_core {
int secureStorageSetJavaEnv(
    JNIEnv *userenv); // call this first when android app begins
int secureStorageWrite(rust::String userkey, rust::String uservalue);
rust::String secureStorageRead(rust::String userkey);
} // namespace defi_wallet_core
} // namespace org
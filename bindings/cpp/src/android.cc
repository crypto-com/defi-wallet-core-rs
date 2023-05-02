// Copyright 2022, Cronos Labs. All Rights Reserved

#ifdef __ANDROID__
#include <jni.h>
#include <string>
#include "defi-wallet-core-cpp/include/android.h"
#define SECURE_STORAGE_CLASS "com/cronos/play/SecureStorage"
using namespace std;

JNIEnv *g_env = NULL;
namespace org {
namespace defi_wallet_core {

int secureStorageSetJavaEnv(JNIEnv *userenv) {
  g_env = userenv;
  return 1;
}

jclass getSecureStorageClass(JNIEnv *env) {
  string secureStorageClass = SECURE_STORAGE_CLASS;
  jclass kotlinClass = env->FindClass(secureStorageClass.c_str());
  return kotlinClass;
}
jobject getContext(JNIEnv *env) {
  jclass activityThreadClass = env->FindClass("android/app/ActivityThread");
  jmethodID currentActivityThreadMethod =
      env->GetStaticMethodID(activityThreadClass, "currentActivityThread",
                             "()Landroid/app/ActivityThread;");
  jobject activityThread = env->CallStaticObjectMethod(
      activityThreadClass, currentActivityThreadMethod);
  jmethodID getApplicationMethod = env->GetMethodID(
      activityThreadClass, "getApplication", "()Landroid/app/Application;");
  jobject context = env->CallObjectMethod(activityThread, getApplicationMethod);
  return context;
}
int secureStorageWriteBasic(JNIEnv *env, string userkey, string uservalue) {

  jobject context = getContext(env);
  jclass secureStorageClass = getSecureStorageClass(env);
  jmethodID functionMethod = env->GetStaticMethodID(
      secureStorageClass, "writeSecureStorage",
      "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;)I");
  jstring key = env->NewStringUTF(userkey.c_str());
  jstring value = env->NewStringUTF(uservalue.c_str());
  jint ret = env->CallStaticIntMethod(secureStorageClass, functionMethod,
                                      context, key, value);

  return (int)ret;
}

string secureStorageReadBasic(JNIEnv *env, string userkey) {
  jobject context = getContext(env);
  jclass secureStorageClass = getSecureStorageClass(env);
  jmethodID functionMethod = env->GetStaticMethodID(
      secureStorageClass, "readSecureStorage",
      "(Landroid/content/Context;Ljava/lang/String;)Ljava/lang/String;");
  jstring userkeyarg = env->NewStringUTF(userkey.c_str());
  jobject ret = env->CallStaticObjectMethod(secureStorageClass, functionMethod,
                                            context, userkeyarg);
  string retstring = string(env->GetStringUTFChars((jstring)ret, 0));

  return retstring;
}

int secureStorageWrite(rust::String userkey, rust::String uservalue) {
  try {
    JNIEnv *env = g_env;
    string userkeystring = userkey.c_str();
    string uservaluestring = uservalue.c_str();
    int ret = secureStorageWriteBasic(env, userkeystring, uservaluestring);
    return ret;
  } catch (exception &e) {
    return 0; // fail
  }
}

rust::String secureStorageRead(rust::String userkey) {
  try {
    JNIEnv *env = g_env;
    string ret = secureStorageReadBasic(env, userkey.c_str());
    return rust::String(ret.c_str());
  } catch (exception &e) {
    return rust::String(""); // fail
  }
}

} // namespace defi_wallet_core
} // namespace org
#endif

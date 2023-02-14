package com.cronos.play

import android.content.Context
import android.util.Log
import androidx.security.crypto.EncryptedFile
import androidx.security.crypto.MasterKey
import androidx.security.crypto.MasterKey.Builder
import java.io.File

class SecureStorage {

    companion object {
        init {}

        // read from secure storage
        // return json string
        // success: {"result":"value","success":"1","error":""}
        // fail: {"result":"","success":"0","error":"encrypt file not found"}
        @JvmStatic
        fun readSecureStorage(context: Context, key: String): String {
            try {
                val masterKey: MasterKey =
                    Builder(context).setKeyScheme(MasterKey.KeyScheme.AES256_GCM).build()

                val file = File(context.filesDir, key)
                val encryptedFile: EncryptedFile =
                    EncryptedFile.Builder(
                        context,
                        file,
                        masterKey,
                        EncryptedFile.FileEncryptionScheme.AES256_GCM_HKDF_4KB,
                    )
                        .build()

                val ret = if (file.exists()) {
                    encryptedFile.openFileInput().use { inputStream ->
                        val myvalue = inputStream.readBytes().toString(Charsets.UTF_8)
                        "{\"result\":\"$myvalue\",\"success\":\"1\",\"error\":\"\"}"
                    }
                } else {
                    "{\"result\":\"\",\"success\":\"0\",\"error\":\"encrypt file not found\"}"
                }

                return ret
            } catch (e: Throwable) {
                Log.e("readSecureStorage exception error", "Failed to read secure storage: $e")
                return "{\"result\":\"\",\"success\":\"0\",\"error\":\"$e\"}"
            }
        }

        // write to secure storage
        // return 1: success, 0: fail
        @JvmStatic
        fun writeSecureStorage(context: Context, key: String, value: String): Int {
            try {
                val masterKey: MasterKey =
                    Builder(context).setKeyScheme(MasterKey.KeyScheme.AES256_GCM).build()

                val file = File(context.filesDir, key)
                val encryptedFile: EncryptedFile =
                    EncryptedFile.Builder(
                        context,
                        file,
                        masterKey,
                        EncryptedFile.FileEncryptionScheme.AES256_GCM_HKDF_4KB,
                    )
                        .build()

                var ret = 0
                if (file.exists()) {
                    file.delete()
                }

                encryptedFile.openFileOutput().use { outputStream ->
                    outputStream.write(value.toByteArray())
                    ret = 1
                }

                return ret
            } catch (e: Throwable) {
                Log.e("writeSecureStorage exception error", "Failed to write secure storage: $e")
                return 0
            }
        }
    }
}

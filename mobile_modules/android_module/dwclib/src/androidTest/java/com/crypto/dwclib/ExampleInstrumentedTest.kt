package com.crypto.dwclib

import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.defi.wallet.core.common.*

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {
    @Test
    fun useAppContext() {
        // Context of the app under test.
        val appContext = InstrumentationRegistry.getInstrumentation().targetContext
        assertEquals("com.crypto.dwclib.test", appContext.packageName)
    }

    @Test
    fun mnemonicTest() {
        val wallet = HdWallet.generateWallet("")
        val mnemonic = wallet.getBackupMnemonicPhrase()
    }

    @Test
    fun getAccountBalanceTest() {
        val rb = getAccountBalanceBlocking("https://mainnet.crypto.org:1317","cro1yjjlx5qsrj5rxn5xtd5rkm6dcqzlchxkrvsmg6","basecro",BalanceApiVersion.NEW)
    }
}
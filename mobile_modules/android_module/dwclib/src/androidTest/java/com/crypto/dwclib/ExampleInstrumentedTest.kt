package com.crypto.dwclib

import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.defi.wallet.core.common.*
import org.junit.Assert

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
    fun mnemonicWorkTest() {
        var wallet = HdWallet.generateWallet("", MnemonicWordCount.TWENTY_FOUR)
        var mnemonic = wallet.getBackupMnemonicPhrase()
        println("words 24:" + mnemonic)
        wallet = HdWallet.generateWallet("", MnemonicWordCount.EIGHTEEN)
        mnemonic = wallet.getBackupMnemonicPhrase()
        println("words 18:" + mnemonic)
        wallet = HdWallet.generateWallet("", MnemonicWordCount.TWELVE)
        mnemonic = wallet.getBackupMnemonicPhrase()
        println("words 12:" + mnemonic)
        wallet.destroy()
    }

    @Test
    fun mnemonic12WordsTest() {
        val words = "lumber flower voice hood obvious behave relax chief warm they they mountain"
        val wallet = HdWallet.recoverWallet(words,"")
        var priv = wallet.getKey("m/84'/0'/0'/0/0")
        Assert.assertEquals(priv.toHex(),"fb3c49bbf7285e9001481586307b1a72ccd8a3fc2256816395ca1c3f0c8373e5")
        priv = wallet.getKey("m/44'/60'/0'/1/5")
        Assert.assertEquals(priv.toHex(),"26b940f6f077860c2113d1e1b4607610f83402f127abf47340e2b9c81d76729a")
        priv = wallet.getKey("m/44'/118'/2'/5/10")
        Assert.assertEquals(priv.toHex(),"2b013bcc85c6a29caf4b05a0389be056e33803e56e62c9b7f8e3d0e09056549a")
    }

    fun mnemonic24WordsTest() {
        val words = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief"
        val wallet = HdWallet.recoverWallet(words,"")
        var priv = wallet.getKey("m/84'/0'/0'/0/0")
        Assert.assertEquals(priv.toHex(),"fdd3354458335c3f41d08d0c3e12d36128ed9b955a83956fcc2702fa414e2328")
        priv = wallet.getKey("m/44'/60'/0'/1/5")
        Assert.assertEquals(priv.toHex(),"da40002e28565270fa6855239771703c3f9bcd16dedf430006f6dea43049fe19")
        priv = wallet.getKey("m/44'/118'/2'/5/10")
        Assert.assertEquals(priv.toHex(),"056f7155c542569fb32bd43519890adb92ed831cadb259b67e50c21c758279501")
    }

    @Test
    fun getAccountBalanceTest() {
        val rb = getAccountBalanceBlocking(
            "https://mainnet.crypto.org:1317",
            "cro1yjjlx5qsrj5rxn5xtd5rkm6dcqzlchxkrvsmg6",
            "basecro",
            BalanceApiVersion.NEW
        )
        println("Balance:" + rb)
    }


}
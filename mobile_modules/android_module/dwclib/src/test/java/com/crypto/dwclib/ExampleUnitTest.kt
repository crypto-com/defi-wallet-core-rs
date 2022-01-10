package com.crypto.dwclib

import org.junit.Test

import org.junit.Assert.*
import com.defi.wallet.core.common.*

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
class ExampleUnitTest {
    @Test
    fun getBalanceTest() {
        val rb = getAccountBalanceBlocking("https://mainnet.crypto.org:1317","cro1yjjlx5qsrj5rxn5xtd5rkm6dcqzlchxkrvsmg6","basecro",BalanceApiVersion.NEW)
        println(rb.amount)
    }
}
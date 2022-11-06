package com.example.android_example

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView
import com.defi.wallet.core.common.*

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val textview = findViewById<TextView>(R.id.textview)

        val wallet = HdWallet.generateWallet("",MnemonicWordCount.TWELVE)
        val mnemonic = wallet.getBackupMnemonicPhrase()
        var text = "mnemonic:" + mnemonic + "\n"
        val address = wallet.getDefaultAddress(WalletCoin.CosmosSdk(Network.CosmosHub))
        text += "address: " + address + "\n"

        val priv = wallet.getKey("m/44'/118'/0'/0/0")
        val txinfo = CosmosSdkTxInfo(
            1UL,
            1UL,
            100UL,
            SingleCoin.Atom(100UL),
            9000U,
            "memo",
            Network.CosmosHub
        )
        val siged_tx = buildSignedSingleMsgTx(
            txinfo,
            CosmosSdkMsg.BankSend(
                "cosmos1rw2cc4hj6ahfk87pqh9znjzgmqwq8ec8nzt0e9",
                SingleCoin.Atom(20UL)
            ),
            priv
        )

        text += "siged_tx: "
        for (b in siged_tx) {
            print(b)
            val h = String.format("%02x", b.toInt())
            text += h
        }

        val rb = getAccountBalanceBlocking("https://mainnet.crypto.org:1317","cro1yjjlx5qsrj5rxn5xtd5rkm6dcqzlchxkrvsmg6","basecro")
        text += "\namount: " + rb.amount

        text += "\nend"
        textview.text = text
    }
}

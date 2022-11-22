import SwiftUI
import dwclib

struct ContentView: View {
    var body: some View {
        var wallet = try? HdWallet.generateWallet(password: "", wordCount: MnemonicWordCount.twelve)
        var mnemonic = try? wallet?.getBackupMnemonicPhrase()
        Text("mnemonic: " + mnemonic!)
        
        var address = try? wallet?.getDefaultAddress(coin: WalletCoin.cosmosSdk(network: Network.cosmosHub))
        Text("address: " + address!)
        var priv = try? wallet?.getKey(derivationPath: "m/44'/118'/0'/0/0")

        var txinfo = CosmosSdkTxInfo.init(accountNumber: 1, sequenceNumber: 1, gasLimit: 100, feeAmount: SingleCoin.atom(amount: 100), timeoutHeight: 9000, memoNote: "memo", network: Network.cosmosHub)
        
        var siged_tx = try? buildSignedSingleMsgTx(txInfo: txinfo, msg: CosmosSdkMsg.bankSend(recipientAddress: "cosmos1rw2cc4hj6ahfk87pqh9znjzgmqwq8ec8nzt0e9", amount: SingleCoin.atom(amount: 20)), secretKey: priv!)
        
        
        let hexArray = siged_tx?.compactMap { String(format: "%02x", $0).lowercased()}
        let hexString = hexArray?.joined(separator: "")
        Text("siged_tx: " + hexString!)

        var br = try? getAccountBalanceBlocking(apiUrl: "https://mainnet.crypto.org:9090", address: "cro1yjjlx5qsrj5rxn5xtd5rkm6dcqzlchxkrvsmg6", denom: "basecro", version: BalanceApiVersion.new)
        Text("balance: " + br!.amount)
        Text("end")
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

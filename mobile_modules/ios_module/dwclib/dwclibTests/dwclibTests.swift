//
//  dwclibTests.swift
//  dwclibTests
//
//  Created by Hao Zhang on 2022/2/10.
//

import XCTest
@testable import dwclib

class dwclibTests: XCTestCase {
    let words24 = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief"

    func txinfo1() -> CosmosSdkTxInfo {
        var txinfo = CosmosSdkTxInfo(
            accountNumber: 1, sequenceNumber: 0, gasLimit: 100000, feeAmount: SingleCoin.atom(amount: 1), timeoutHeight: 9001, memoNote: "", network: Network.cosmosHub
        )
        return txinfo
    }

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
        // Any test you write for XCTest can be annotated as throws and async.
        // Mark your test throws to produce an unexpected failure when your test encounters an uncaught error.
        // Mark your test async to allow awaiting for asynchronous code to complete. Check the results with assertions afterwards.
    }
    
    func testHdWalletWork() throws {
        var wallet = try? HdWallet.generateWallet(password: "",wordCount: MnemonicWordCount.twelve)
        var mnemonic = wallet?.getBackupMnemonicPhrase()
        print(mnemonic)
        assert(true, "HdWalletWork error")
    }
    
    func testSingleMsgSignPayloadWork() throws {
        var wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words24, password: "")
        var priv = try? wallet?.getKey(derivationPath: "m/44'/118'/0'/0/0")
        var pubKeyHex = bytesToHex(data: (priv?.getPublicKeyBytes())!)
        print("pubKeyHex:",pubKeyHex)
        assert(pubKeyHex == "028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c520", "pubKeyHex error")
        
        var payload = try? getSingleMsgSignPayload(txInfo: txinfo1(), msg: CosmosSdkMsg.bankSend(recipientAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", amount: SingleCoin.atom(amount: 1)), senderPubkey: (priv?.getPublicKeyBytes())!)
        print("payload:",payload)
        
        var sigedTx = try? buildSignedSingleMsgTx(txInfo: txinfo1(), msg: CosmosSdkMsg.bankSend(recipientAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", amount: SingleCoin.atom(amount: 1)), secretKey: priv!)
        print("sigedTx:",sigedTx)
    }

    func testBuildSignedMsgTxWork() throws {
        var wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words24, password: "")
        var priv = try? wallet?.getKey(derivationPath: "m/44'/118'/0'/0/0")
        var sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: [CosmosSdkMsg.bankSend(recipientAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", amount: SingleCoin.atom(amount: 1)),CosmosSdkMsg.bankSend(recipientAddress: "cosmos1a83x94xww47e32rgpytttucx2vexxcn2lc2ekx", amount: SingleCoin.atom(amount: 2))], secretKey: priv!)
        print("sigedTx:",sigedTx)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}

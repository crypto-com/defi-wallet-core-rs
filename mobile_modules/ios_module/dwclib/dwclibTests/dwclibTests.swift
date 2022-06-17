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
    
    func testWallet() throws {
        // Create wallet with words
        let words = "guard input oyster oyster slot doctor repair shed soon assist blame power"
        let wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words, password: "")
        var mnemonic = wallet?.getBackupMnemonicPhrase()
        assert(words == mnemonic)
        
        var address = try? wallet?.getDefaultAddress(coin: WalletCoin.cosmosSdk(network: Network.cryptoOrgMainnet))
        assert(address == "cro16edxe89pn8ly9c7cy702x9e62fdvf3k9tnzycj")
        
        // get address with index
        address = try? wallet?.getAddress(coin: WalletCoin.cosmosSdk(network: Network.cryptoOrgMainnet), index:1 )
        assert(address == "cro1keycl6d55fnlzwgfdufl53vuf95uvxnry6uj2q")
        
        address = try? wallet?.getAddress(coin: WalletCoin.ethereum(network: EthNetwork.mainnet), index: 1)
        assert(address == "0x74aeb73c4f6c10750bcd8608b0347f3e4750151c")
        
        // get key with path
        var priv = try? wallet?.getKey(derivationPath: "m/44'/394'/0'/0/0")
        assert(
            priv?.toHex() == "2e9c6bc5d8df5177697e90e87bd098d2d6165f096195d78f76cca1cecbf37525"
        )
        
        // parse key from hex
        priv = try? SecretKey.fromHex(hex: "e7de4e2f72573cf3c6e1fa3845cec6a4e2aac582702cac14bb9da0bb05aa24ae")
        assert(
            priv?.getPublicKeyHex() ==
            "03cefab3f89c62ecc54c09634516bb2819d20d83757956c7f4690dc3b806ecc7d2"
        )
        
        priv = try? SecretKey.fromHex(hex: "24e585759e492f5e810607c82c202476c22c5876b10247ebf8b2bb7f75dbed2e")
        assert(
            priv?.getPublicKeyHex() ==
            "02059b1fc4b7834d77765a024b6c52f570f19ed5113d8cedea0b90fbae39edda1c"
        )
        
        // get address from private key
        address = try? priv?.toAddress(coin: WalletCoin.ethereum(network: EthNetwork.mainnet))
        assert(
            address ==
            "0x714e0ed767d99f8be2b789f9dd1e2113de8eac53"
        )
    }
    
    func testCosmos() throws {
        let wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words24, password: "")
        let priv = try? wallet?.getKey(derivationPath: "m/44'/118'/0'/0/0")
        
        // bank send transaction
        var msgList = [
            CosmosSdkMsg.bankSend(recipientAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", amount: SingleCoin.atom(amount: 1))
        ]
        var sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        var sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a96010a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d12073130303030303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40aa554d4be2ac72d644002296882c188de39944efd21fc021bf1202721fff40d05e9c86d398b11bb94e16cf79dd4866eca22d84b6785bd0098ed353615585485c"
        )
        
        // muti message transaction
        msgList = [
            CosmosSdkMsg.bankSend(recipientAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", amount: SingleCoin.atom(amount: 1)),
            CosmosSdkMsg.bankSend(recipientAddress: "cosmos1a83x94xww47e32rgpytttucx2vexxcn2lc2ekx", amount: SingleCoin.atom(amount: 2))
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aa9020a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a1a100a057561746f6d1207313030303030300a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73316138337839347877773437653332726770797474747563783276657878636e326c6332656b781a100a057561746f6d12073230303030303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a406be1c153eda9e3ba022d2e9138c0682991ba6cf6b8b7bdc75ae1adb88b8a977b35e18292b569cb66ffff16189f37a5848648f14caa1084cfb4f7041deda737ae"
        )
        
        // stake delegate transaction
        msgList = [
            CosmosSdkMsg.stakingDelegate(validatorAddress: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3", amount: SingleCoin.uatom(amount: 100))
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aa0010a9a010a232f636f736d6f732e7374616b696e672e763162657461312e4d736744656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a404d71f59fb847a319b5cd4a831eed8c9baa4051a656392be6c981f95d5debf552011318ac433caf47e8df57d6fb133cf9f5d91db031dff59beb2d98b7e041a125"
        )
        
        // stake undelegate transaction
        msgList = [
            CosmosSdkMsg.stakingUndelegate(validatorAddress: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3", amount: SingleCoin.uatom(amount: 100))
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aa2010a9c010a252f636f736d6f732e7374616b696e672e763162657461312e4d7367556e64656c656761746512730a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b331a0c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a407c468b64e58510b3dc20259d6042f280b8ee9e9aca6a0b3bfc21d931509659b70169aad7543970b65c8bc6aa3bccbb8868ce85d3eece042396492e6dc666404a"
        )
        
        // stake begin redelegate transaction
        msgList = [
            CosmosSdkMsg.stakingBeginRedelegate(validatorSrcAddress: "cosmosvaloper1l5s7tnj28a7zxeeckhgwlhjys8dlrrefd5hqdp", validatorDstAddress: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3", amount: SingleCoin.uatom(amount: 100))
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0ade010ad8010a2a2f636f736d6f732e7374616b696e672e763162657461312e4d7367426567696e526564656c656761746512a9010a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572316c357337746e6a323861377a786565636b6867776c686a797338646c727265666435687164701a34636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b33220c0a057561746f6d120331303018a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40de252fd4e12b786c499d62ea5cc7070899acff3b88d6438c5542529a4a18d15755496029a1936865658b872ec9765d92a8394bad2443da84e73536917a65139f"
        )
        
        // distribution set withdraw address transaction
        msgList = [
            CosmosSdkMsg.distributionSetWithdrawAddress(withdrawAddress: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a9a010a94010a322f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367536574576974686472617741646472657373125e0a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a122d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40c29ab82aec56651fb33a4df92f499bb4624d0be31cd51d64df234a4d380282bb5ebda7aa54a84d8075f6b2ffb0b5fa5f98118b108888fcfdbbaf4efaca4ffdba"
        )
        
        // distribution set withdraw delegator reward transaction
        msgList = [
            CosmosSdkMsg.distributionWithdrawDelegatorReward(validatorAddress: "cosmosvaloper19dyl0uyzes4k23lscla02n06fc22h4uq4e64k3")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aa6010aa0010a372f636f736d6f732e646973747269627574696f6e2e763162657461312e4d7367576974686472617744656c656761746f7252657761726412650a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a1234636f736d6f7376616c6f706572313964796c3075797a6573346b32336c73636c6130326e30366663323268347571346536346b3318a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a40ae166e9cc8489ded5e6dc82e99d0b7ee017fc0234a70c0851cff133c811e92165391c5404c474278ed8cbe85b28f1cf4ee6e59071ccdf3d495dddfd12c4029f1"
        )
        
        // nft issue denom transaction
        msgList = [
            CosmosSdkMsg.nftIssueDenom(id: "edition01", name: "domingo1", schema: "test")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a720a6d0a1f2f636861696e6d61696e2e6e66742e76312e4d7367497373756544656e6f6d124a0a0965646974696f6e30311208646f6d696e676f311a0474657374222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a404d0eb09d0735c80d8dfa9a7113beeff4dc38fb6f6bdfcad1a39ff0153ba5eaa3236d8413abcd31c62755946238656b80df428c7d05b43fcff3531dfae7687064"
        )
        
        // nft transfer transaction
        msgList = [
            CosmosSdkMsg.nftTransfer(id: "edition01", denomId: "domingo1", recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a9d010a97010a202f636861696e6d61696e2e6e66742e76312e4d73675472616e736665724e465412730a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a222d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a409645a66de4809f282349fce4a80f8478d78b0b0c0d8d23f4ebe7430589fed7123e0e432f244e7b991130a475db8e2d5f90ae5f933682763afea798f78da156ff"
        )
        
        // nft mint transaction
        msgList = [
            CosmosSdkMsg.nftMint(id: "edition01", denomId: "domingo1", name: "test", uri: "test", data: "test", recipient: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aac010aa6010a1c2f636861696e6d61696e2e6e66742e76312e4d73674d696e744e46541285010a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a3a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a401a3eb24123103ee0ec2856315311b8c9c01e3e54249beb18bec91864834c6ffd7605e2a866fa7307f2786bc15e9075fa8d73cd188924eb7bded6214c858f9fdf"
        )
        
        // nft edit transaction
        msgList = [
            CosmosSdkMsg.nftEdit(id: "edition01", denomId: "domingo1", name: "test", uri: "test", data: "test")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a7b0a760a1c2f636861696e6d61696e2e6e66742e76312e4d7367456469744e465412560a0965646974696f6e30311208646f6d696e676f311a04746573742204746573742a0474657374322d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a401134c4d5d9c1c6f5435e2dcc701512401c4220249b54ffc7c0e6793311399e9d60207caf1c175cbfc6ab999c7d8e75ef5f66931f73829e03f1ea9d3987bf442e"
        )
        
        // nft burn transaction
        msgList = [
            CosmosSdkMsg.nftBurn(id: "edition01", denomId: "domingo1")
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0a690a640a1c2f636861696e6d61696e2e6e66742e76312e4d73674275726e4e465412440a0965646974696f6e30311208646f6d696e676f311a2d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a18a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a4046e4de5a3c55bd27c2e359315e9b52bb684cc0c3e9d470e77a4d922a1bf2c1b334b3504ce639cc94ed84f403f5af4878ae4efea3a696caf9da49597bed2717d9"
        )
        
        // ibc transfer transaction
        msgList = [
            CosmosSdkMsg.ibcTransfer(receiver: "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", sourcePort: "transfer", sourceChannel: "channel-3", token: SingleCoin.other(amount: "100000000", denom: "basetcro"), timeoutHeight: Height.init(revisionNumber: 0, revisionHeight: 0), timeoutTimestamp: 1645800000000000000)
        ]
        sigedTx = try? buildSignedMsgTx(txInfo: txinfo1(), msgs: msgList, secretKey: priv!)
        sigedTxHex = bytesToHex(data: sigedTx!)
        assert(
            sigedTxHex ==
            "0aca010ac4010a292f6962632e6170706c69636174696f6e732e7472616e736665722e76312e4d73675472616e736665721296010a087472616e7366657212096368616e6e656c2d331a150a08626173657463726f1209313030303030303030222d636f736d6f73316c357337746e6a323861377a786565636b6867776c686a797338646c7272656667717234706a2a2d636f736d6f73313964796c3075797a6573346b32336c73636c6130326e3036666332326834757173647771367a3200388080da9a95ccc3eb1618a94612680a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a21028c3956de0011d6b9b2c735045647d14b38e63557e497fc025de9a17a5729c52012040a02080112160a100a057561746f6d12073130303030303010a08d061a409cee761ef007f4e0020dc1fe85610affd7555227e15cd068a364659ed58b638e725f543da0e1c6e8d39076ea9400de778053650053cbf2c98f3f72499938b97d"
        )
        
    }
    
    func testEthSignTransactionLegacy() throws {
        let jsonStr =
        "{\"from\":\"0x68418d0fdb846e8736aa613159035a9d9fde11f0\",\"to\":\"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d\",\"gas\":\"0x5208\",\"gasPrice\":\"0x5f5e100\",\"value\":\"0xde0b6b3a7640000\",\"data\":\"0x\",\"nonce\":\"0x0\",\"chainId\":\"0x0539\"}"
        let priv = try? SecretKey.fromHex(hex: "6f53576748877b603718b1aa1e7106aec5e15c1a2f39ea8c4683ac0d5a435a13")
        let rawTx = try? ethSignTransactionWithChainid(jsonStr: jsonStr, secretKey: priv!, chainId: 1337)
        let rawTxHex = bytesToHex(data: rawTx!)
        assert(
            rawTxHex ==
            "f86d808405f5e100825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080820a96a0dd110c3396ac52d7a23db8e5cca23b42983636192190baeec2178d5b33b02369a057ace20b2e326e7e24b0e1ca57d312b19a29a8353301d2280e5a829fa7866f10"
        )
    }
    
    func testEthSignTransactionEip2930() throws {
        let jsonStr =
        "{\"from\":\"0x68418d0fdb846e8736aa613159035a9d9fde11f0\",\"to\":\"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d\",\"gas\":\"0x5208\",\"gasPrice\":\"0x5f5e100\",\"value\":\"0xde0b6b3a7640000\",\"data\":\"0x\",\"nonce\":\"0x0\",\"accessList\":[{\"address\":\"0x0000000000000000000000000000000000000000\",\"storageKeys\":[\"0x0000000000000000000000000000000000000000000000000000000000000000\"]}],\"chainId\":\"0x0539\"}"
        let priv = try? SecretKey.fromHex(hex: "6f53576748877b603718b1aa1e7106aec5e15c1a2f39ea8c4683ac0d5a435a13")
        let rawTx = try? ethSignTransactionWithChainid(jsonStr: jsonStr, secretKey: priv!, chainId: 1337)
        let rawTxHex = bytesToHex(data: rawTx!)
        assert(
            rawTxHex ==
            "01f8a8820539808405f5e100825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080f838f7940000000000000000000000000000000000000000e1a0000000000000000000000000000000000000000000000000000000000000000080a024117c04934ced6c3d272447816f0ebc00e97dd012f8d3872d661a48152c0e5ca0601c21637bad2f399da6a7e314a6119956f4bb8c2d7dd2df6905786e56a35c47"
        )
    }
    
    func testEthSignTransactionEip1559() throws {
        // eip1559 build signed raw transaction
        var jsonStr =
        "{\"from\":\"0x68418d0fdb846e8736aa613159035a9d9fde11f0\",\"to\":\"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d\",\"gas\":\"0x5208\",\"value\":\"0xde0b6b3a7640000\",\"data\":\"0x\",\"nonce\":\"0x0\",\"maxPriorityFeePerGas\":\"0x1\",\"maxFeePerGas\":\"0x77359401\",\"chainId\":\"0x0539\"}"
        let priv = try? SecretKey.fromHex(hex: "6f53576748877b603718b1aa1e7106aec5e15c1a2f39ea8c4683ac0d5a435a13")
        var rawTx = try? ethSignTransaction(jsonStr: jsonStr, secretKey: priv!)
        var rawTxHex = bytesToHex(data: rawTx!)
        assert(
            rawTxHex ==
            "02f87082053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080c001a0caa0df6665a08e4fae0839395387aabeeef4064134dd09a771eed6e41d6c258da07817000d01107a554e8e885c872a672df50e2dc25ed5068b83a93e2a27982bce"
        )
        
        // Sign with the specified chainid
        rawTx = try? ethSignTransactionWithChainid(jsonStr: jsonStr, secretKey: priv!, chainId: 1337)
        rawTxHex = bytesToHex(data: rawTx!)
        assert(
            rawTxHex ==
            "02f87082053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080c001a0caa0df6665a08e4fae0839395387aabeeef4064134dd09a771eed6e41d6c258da07817000d01107a554e8e885c872a672df50e2dc25ed5068b83a93e2a27982bce"
        )
        
        jsonStr =
        "{\"from\":\"0x68418d0fdb846e8736aa613159035a9d9fde11f0\",\"to\":\"0x4592d8f8d7b001e72cb26a73e4fa1806a51ac79d\",\"gas\":\"0x5208\",\"value\":\"0xde0b6b3a7640000\",\"data\":\"0x\",\"nonce\":\"0x0\",\"accessList\":[{\"address\":\"0x0000000000000000000000000000000000000000\",\"storageKeys\":[\"0x0000000000000000000000000000000000000000000000000000000000000000\"]}],\"maxPriorityFeePerGas\":\"0x1\",\"maxFeePerGas\":\"0x77359401\",\"chainId\":\"0x0539\"}"
        rawTx = try? ethSignTransactionWithChainid(jsonStr: jsonStr, secretKey: priv!, chainId: 1337)
        rawTxHex = bytesToHex(data: rawTx!)
        assert(
            rawTxHex ==
            "02f8a982053980018477359401825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a764000080f838f7940000000000000000000000000000000000000000e1a0000000000000000000000000000000000000000000000000000000000000000080a0462c27c0ae0a8a2fd448ab299d519823c7016c280881c38747dcda913dc1c4caa0685acccb1f37f87250e9688e805725f2eb0e9f63b53fe311f9ed485f07987cf4"
        )
    }
    
    func testEthSignTypedData() throws {
        var words = "lumber flower voice hood obvious behave relax chief warm they they mountain"
        let wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words, password: "")
        let priv = try? wallet?.getKeyFromIndex(coin: WalletCoin.ethereum(network: EthNetwork.mainnet), index: 0)
        
        let address = try? wallet?.getAddress(coin: WalletCoin.ethereum(network: EthNetwork.mainnet), index: 0)
        assert(address == "0x45f508caf79cb329a46f1757f3526faf8c6b2ea5")
        
        let ethSigner = EthSigner.init(secretKey: priv!)
        var signature = try? ethSigner.signTypedData(jsonTypedData:"{\"types\":{\"EIP712Domain\":[{\"name\":\"name\",\"type\":\"string\"},{\"name\":\"version\",\"type\":\"string\"},{\"name\":\"chainId\",\"type\":\"uint256\"},{\"name\":\"verifyingContract\",\"type\":\"address\"}],\"Person\":[{\"name\":\"name\",\"type\":\"string\"},{\"name\":\"wallet\",\"type\":\"address\"}]},\"primaryType\":\"Person\",\"domain\":{\"name\":\"Ether Person\",\"version\":\"1\",\"chainId\":1,\"verifyingContract\":\"0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC\"},\"message\":{\"name\":\"Bob\",\"wallet\":\"0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB\"}}")
        assert(
            signature ==
            "0xb3c346815d16ca57eb710ddcfb50f08c0db2d5c0c7a8976bc28ad3642696e7ac533725167b3bd3e2460b577af5737368b6ab9c37dd9e80689103467acd0ff12c1c"
        )
        
        signature = try? ethSigner.signTypedData(jsonTypedData:"{\"types\":{\"EIP712Domain\":[{\"type\":\"address\",\"name\":\"verifyingContract\"}],\"SafeTx\":[{\"type\":\"address\",\"name\":\"to\"},{\"type\":\"uint256\",\"name\":\"value\"},{\"type\":\"bytes\",\"name\":\"data\"},{\"type\":\"uint8\",\"name\":\"operation\"},{\"type\":\"uint256\",\"name\":\"safeTxGas\"},{\"type\":\"uint256\",\"name\":\"baseGas\"},{\"type\":\"uint256\",\"name\":\"gasPrice\"},{\"type\":\"address\",\"name\":\"gasToken\"},{\"type\":\"address\",\"name\":\"refundReceiver\"},{\"type\":\"uint256\",\"name\":\"nonce\"}]},\"domain\":{\"verifyingContract\":\"0x25a6c4BBd32B2424A9c99aEB0584Ad12045382B3\"},\"primaryType\":\"SafeTx\",\"message\":{\"to\":\"0x9eE457023bB3De16D51A003a247BaEaD7fce313D\",\"value\":\"20000000000000000\",\"data\":\"0x\",\"operation\":0,\"safeTxGas\":27845,\"baseGas\":0,\"gasPrice\":\"0\",\"gasToken\":\"0x0000000000000000000000000000000000000000\",\"refundReceiver\":\"0x0000000000000000000000000000000000000000\",\"nonce\":3}}")
        assert(
            signature ==
            "0xc3080f1573b93b5a7b942f6595fa79186762a400ef308d3adcb63d3ba5bc275069ddeb75ec5bb6a7391e17aceb45daacb81c9711cacc294deff7411a119fa7bd1b"
        )
        
        signature = try? ethSigner.signTypedData(jsonTypedData:"{\"types\":{\"EIP712Domain\":[{\"name\":\"chainId\",\"type\":\"uint256\"},{\"name\":\"name\",\"type\":\"string\"},{\"name\":\"verifyingContract\",\"type\":\"address\"},{\"name\":\"version\",\"type\":\"string\"}],\"Action\":[{\"name\":\"action\",\"type\":\"string\"},{\"name\":\"params\",\"type\":\"string\"}],\"Cell\":[{\"name\":\"capacity\",\"type\":\"string\"},{\"name\":\"lock\",\"type\":\"string\"},{\"name\":\"type\",\"type\":\"string\"},{\"name\":\"data\",\"type\":\"string\"},{\"name\":\"extraData\",\"type\":\"string\"}],\"Transaction\":[{\"name\":\"DAS_MESSAGE\",\"type\":\"string\"},{\"name\":\"inputsCapacity\",\"type\":\"string\"},{\"name\":\"outputsCapacity\",\"type\":\"string\"},{\"name\":\"fee\",\"type\":\"string\"},{\"name\":\"action\",\"type\":\"Action\"},{\"name\":\"inputs\",\"type\":\"Cell[]\"},{\"name\":\"outputs\",\"type\":\"Cell[]\"},{\"name\":\"digest\",\"type\":\"bytes32\"}]},\"primaryType\":\"Transaction\",\"domain\":{\"chainId\":\"56\",\"name\":\"da.systems\",\"verifyingContract\":\"0x0000000000000000000000000000000020210722\",\"version\":\"1\"},\"message\":{\"DAS_MESSAGE\":\"SELL mobcion.bit FOR 100000 CKB\",\"inputsCapacity\":\"1216.9999 CKB\",\"outputsCapacity\":\"1216.9998 CKB\",\"fee\":\"0.0001 CKB\",\"digest\":\"0x53a6c0f19ec281604607f5d6817e442082ad1882bef0df64d84d3810dae561eb\",\"action\":{\"action\":\"start_account_sale\",\"params\":\"0x00\"},\"inputs\":[{\"capacity\":\"218 CKB\",\"lock\":\"das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...\",\"type\":\"account-cell-type,0x01,0x\",\"data\":\"{ account: mobcion.bit, expired_at: 1670913958 }\",\"extraData\":\"{ status: 0, records_hash: 0x55478d76900611eb079b22088081124ed6c8bae21a05dd1a0d197efcc7c114ce }\"}],\"outputs\":[{\"capacity\":\"218 CKB\",\"lock\":\"das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...\",\"type\":\"account-cell-type,0x01,0x\",\"data\":\"{ account: mobcion.bit, expired_at: 1670913958 }\",\"extraData\":\"{ status: 1, records_hash: 0x55478d76900611eb079b22088081124ed6c8bae21a05dd1a0d197efcc7c114ce }\"},{\"capacity\":\"201 CKB\",\"lock\":\"das-lock,0x01,0x051c152f77f8efa9c7c6d181cc97ee67c165c506...\",\"type\":\"account-sale-cell-type,0x01,0x\",\"data\":\"0x1209460ef3cb5f1c68ed2c43a3e020eec2d9de6e...\",\"extraData\":\"\"}]}}"
        )
        assert(
            signature ==
            "0x08067fca7b0f1651669749a61edd478d828c6dc6112fc567595f9e0f58630ea255fb3d83c6a3680d91f0fcb4d602522495ed9d172be5485c88e8921101e5e8ed1b"
        )
        
        signature = try? ethSigner.signTypedData(jsonTypedData:"{\"types\":{\"EIP712Domain\":[{\"type\":\"uint256\",\"name\":\"chainId\"},{\"type\":\"address\",\"name\":\"verifyingContract\"}],\"SafeTx\":[{\"type\":\"address\",\"name\":\"to\"},{\"type\":\"uint256\",\"name\":\"value\"},{\"type\":\"bytes\",\"name\":\"data\"},{\"type\":\"uint8\",\"name\":\"operation\"},{\"type\":\"uint256\",\"name\":\"safeTxGas\"},{\"type\":\"uint256\",\"name\":\"baseGas\"},{\"type\":\"uint256\",\"name\":\"gasPrice\"},{\"type\":\"address\",\"name\":\"gasToken\"},{\"type\":\"address\",\"name\":\"refundReceiver\"},{\"type\":\"uint256\",\"name\":\"nonce\"}]},\"domain\":{\"verifyingContract\":\"0x111dAE35D176A9607053e0c46e91F36AFbC1dc57\",\"chainId\":\"4\"},\"primaryType\":\"SafeTx\",\"message\":{\"to\":\"0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa\",\"value\":\"0\",\"data\":\"0xa9059cbb00000000000000000000000099d580d3a7fe7bd183b2464517b2cd7ce5a8f15a0000000000000000000000000000000000000000000000000de0b6b3a7640000\",\"operation\":0,\"safeTxGas\":0,\"baseGas\":0,\"gasPrice\":\"0\",\"gasToken\":\"0x0000000000000000000000000000000000000000\",\"refundReceiver\":\"0x0000000000000000000000000000000000000000\",\"nonce\":15}}"
        )
        assert(
            signature ==
            "0xa0f3d4c87ded91047185a9e6e36e1ce22ab7dee425067499aed5989feb191e450feab83a0a8ec5d5975bff5bf4fe4a4f6c1293f34d2fecb308f28863a451b0251b"
        )
        
    }
    
    func testEthSign() throws {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain"
        let wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words, password: "")
        let priv = try? wallet?.getKeyFromIndex(coin: WalletCoin.ethereum(network: EthNetwork.mainnet), index: 0)
        
        // eth_sign
        let ethSigner = EthSigner.init(secretKey: priv!)
        var signature = try? ethSigner.ethSignInsecure(hash: "879a053d4800c6354e76c7985a865d2922c82fb5b3f4577b2fe08b998954f2e0")
        assert(
            signature ==
            "0x59e8f544fdee652ae4475a53921ad8030794df66aedf77b218349ba1f476712739caf09dfee2c8ac60e17cc5f2102c09d4ad04de6223a38e9705b28276d71f471b"
        )
        
        // personal sign
        signature = ethSigner.personalSign(message: "Example `personal_sign` message")
        assert(
            signature ==
            "0x1490cd65cdfd5145a2b4e4e562b8c78008cb374ac36b2bbcd6b65dbcc14d31c453c705c4399e745fbf22ccd3939754ff2e4bbbe13a7dacae8a44aeb95f6e68c81b"
        )
    }
    
    func testCosmosSignDirect() throws {
        let words = "lumber flower voice hood obvious behave relax chief warm they they mountain"
        let wallet = try? HdWallet.recoverWallet(mnemonicPhrase: words, password: "")
        let priv = try? wallet?.getKeyFromIndex(coin: WalletCoin.cosmosSdk(network: Network.cosmosHub), index: 0)
        
        let address = try? wallet?.getAddress(coin: WalletCoin.cosmosSdk(network: Network.cosmosHub), index: 0)
        assert(address == "cosmos1ztqcmg76d54d468t6ftkz4zcwwurzz7xhwlsmz")
        
        let auth_info_bytes = "0a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c"
        let body_bytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637"
        
        let cosmosSigner = CosmosSigner.init(secretKey: priv!)
        let signature = try? cosmosSigner.signDirect(chainId: "cosmoshub-4", accountNumber: "1", authInfoBytes: auth_info_bytes, bodyBytes: body_bytes)
        assert(signature == "0a93010a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d12073132333435363712210a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c1a40cc782d8685e320962a3b8379f32119056eab979c7e33f697519c50c0d60aef602c8e97c0155a6e1f99553a5a6bc39e513fe576ce43fa877a459c6c382aa03c2a")
        
    }
    
    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }
    
}

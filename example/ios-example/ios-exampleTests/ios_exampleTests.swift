//
//  ios_exampleTests.swift
//  ios-exampleTests
//
//  Created by Hao Zhang on 2021/12/15.
//

import XCTest
import ios_example

class ios_exampleTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testMnemonic() throws {
        var wallet = try? HdWallet.generateWallet(password: "")
        var mnemonic = try? wallet?.getBackupMnemonicPhrase()
        print(mnemonic)
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}

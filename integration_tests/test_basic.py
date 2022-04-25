#!/usr/bin/env python3

import subprocess
import time
from pathlib import Path

from .utils import ADDRS, CONTRACTS, KEYS, Contract


def test_basic(chainmain, cronos):
    singer1_addr = chainmain.cosmos_cli(0).address("signer1")
    singer2_addr = chainmain.cosmos_cli(0).address("signer2")
    community_addr = chainmain.cosmos_cli(0).address("community")
    delegator1_addr = chainmain.cosmos_cli(0).address("delegator1")
    delegator2_addr = chainmain.cosmos_cli(0).address("delegator2")
    validator_addr = chainmain.cosmos_cli(0).address("validator")
    print(
        singer1_addr,
        singer2_addr,
        community_addr,
        delegator1_addr,
        delegator2_addr,
        validator_addr,
    )

    rpc = chainmain.node_rpc(0)
    print(rpc)

    w3 = cronos.w3
    print(w3.eth.get_block_number())

    print("Signer1 address:", ADDRS["signer1"])

    # ERC20
    erc20 = Contract(
        CONTRACTS["TestERC20"],
        KEYS["signer1"],
    )
    contract = erc20.deploy(w3, 100000000000000000000000000)
    print("ERC20 address:", contract.address)
    print("Name:", contract.functions.name().call())
    print("Symbol:", contract.functions.symbol().call())
    print("Decimals:", contract.functions.decimals().call())
    print("Total Supply:", contract.functions.totalSupply().call())
    print("Balance:", contract.functions.balanceOf(ADDRS["signer1"]).call())

    # ERC721
    erc721 = Contract(
        CONTRACTS["TestERC721"],
        KEYS["signer1"],
    )
    contract = erc721.deploy(w3)
    print("ERC721 address:", contract.address)
    tx = contract.functions.awardItem(
        ADDRS["signer1"], "https://game.example/item-id-8u5h2m.json"
    ).buildTransaction({"from": ADDRS["signer1"]})
    erc721.send(tx)
    print("Name:", contract.functions.name().call())
    print("Symbol:", contract.functions.symbol().call())
    print("Balance:", contract.functions.balanceOf(ADDRS["signer1"]).call())
    print("Owner:", contract.functions.ownerOf(1).call())
    print("tokenURI:", contract.functions.tokenURI(1).call())
    assert contract.functions.balanceOf(ADDRS["signer1"]).call() == 1

    # ERC1155
    erc1155 = Contract(
        CONTRACTS["TestERC1155"],
        KEYS["signer1"],
    )
    contract = erc1155.deploy(w3)
    print("ERC1155 address:", contract.address)
    print("Balance of GOLD:", contract.functions.balanceOf(ADDRS["signer1"], 0).call())
    print("URI of GOLD:", contract.functions.uri(0).call())

    print(
        "Balance of SILVER:", contract.functions.balanceOf(ADDRS["signer1"], 1).call()
    )
    print("URI of SILVER:", contract.functions.uri(1).call())
    print(
        "Balance of THORS_HAMMER:",
        contract.functions.balanceOf(ADDRS["signer1"], 2).call(),
    )
    print("URI of THORS_HAMMER:", contract.functions.uri(2).call())
    print("Balance of SWORD:", contract.functions.balanceOf(ADDRS["signer1"], 3).call())
    print("URI of SWORD:", contract.functions.uri(3).call())
    print(
        "Balance of SHIELD:", contract.functions.balanceOf(ADDRS["signer1"], 4).call()
    )
    print("URI of SHIELD:", contract.functions.uri(4).call())

    # Test cppexamplestatic
    # TODO Pass input and assert output
    cmd = Path(__file__).parent / "../example/cpp-example/cppexamplestatic"

    start = time.time()

    output = subprocess.getoutput(str(cmd))
    print(output)
    if "Assertion failed" in output:
        assert False

    end = time.time()
    print(f"Total Execucute Time: {end} - {start}")

    # TODO Test cppexample

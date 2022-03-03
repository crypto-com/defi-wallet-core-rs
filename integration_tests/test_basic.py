#!/usr/bin/env python3

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

    ERC721 = Contract(
        CONTRACTS["TestERC721"],
        KEYS["validator"],
    )

    contract = ERC721.deploy(w3, "MyTestERC721Token", "MyTestERC721")
    tx = contract.functions.mint(ADDRS["community"], 1).buildTransaction(
        {"from": ADDRS["validator"], "gas": 500000}
    )
    ERC721.send(tx)
    print("Name:", contract.functions.name().call())
    print("Symbol:", contract.functions.symbol().call())
    print("Balance:", contract.functions.balanceOf(ADDRS["community"]).call())
    assert (contract.functions.balanceOf(ADDRS["community"]).call(), 1)

#!/usr/bin/env python3

import tempfile
from pathlib import Path

from network import setup_cronos
from utils import ADDRS, CONTRACTS, KEYS, Contract


def start():
    with tempfile.TemporaryDirectory() as tmp_dir:
        yield from setup_cronos(Path(tmp_dir), 26650)


if __name__ == "__main__":
    cronos = next(start())
    w3 = cronos.w3
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

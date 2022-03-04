#!/usr/bin/env python3

import os
import signal
import tempfile
from pathlib import Path

from pystarport import ports

from network import Cronos, setup_cronos, setup_pystarport, wait_for_port
from utils import ADDRS, CONTRACTS, KEYS, Contract


def cronos():
    pass


if __name__ == "__main__":
    path = Path(tempfile.mkdtemp())
    base_port = 26650
    cfg = Path(__file__).parent / "../scripts/cronos-devnet.yaml"
    proc = setup_pystarport(path, base_port, cfg)
    wait_for_port(ports.evmrpc_port(base_port))
    cronos = Cronos(path / "cronos_777-1")

    w3 = cronos.w3
    ERC721 = Contract(
        CONTRACTS["TestERC721"],
        KEYS["signer1"],
    )

    contract = ERC721.deploy(w3, "MyTestERC721Token", "MyTestERC721")
    tx = contract.functions.mint(ADDRS["signer1"], 1).buildTransaction(
        {"from": ADDRS["validator"], "gas": 500000}
    )
    ERC721.send(tx)
    print("Name:", contract.functions.name().call())
    print("Symbol:", contract.functions.symbol().call())
    print("Balance:", contract.functions.balanceOf(ADDRS["signer1"]).call())

    os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
    # proc.terminate()
    proc.wait()

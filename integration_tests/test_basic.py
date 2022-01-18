#!/usr/bin/env python3
import os
import signal
import subprocess
import time
from pathlib import Path

import requests

from .network import wait_for_port


# WIP
def test_basic(chainmain, cronos):
    singer1_addr = chainmain.cosmos_cli(0).address("signer1")
    print(singer1_addr)

    rpc = chainmain.node_rpc(0)
    print(rpc)

    w3 = cronos.w3
    print(w3.eth.get_block_number())

    cwd = Path(__file__).parent.parent / "example/js-example"
    cmd = ["npm", "start"]
    print(*cmd)
    # WIP: This can not get the webpack process
    proc = subprocess.Popen(
        cmd,
        preexec_fn=os.setsid,
        cwd=cwd,
    )

    time.sleep(20)
    wait_for_port(8080)
    # WIP: It doesn't work. Frontend console log is not retrived in this way.
    r = requests.get("http://localhost:8080/")
    print(r.json())
    print(r.status_code())

    os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
    # proc.terminate()
    proc.wait()

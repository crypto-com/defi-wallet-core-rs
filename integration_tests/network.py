#!/usr/bin/env python3

import json
import os
import signal
import socket
import subprocess
import time
from pathlib import Path

import web3
from pystarport import ports
from pystarport.cosmoscli import CosmosCLI


class Chain:
    def __init__(self, base_dir, cmd):
        self.base_dir = base_dir
        self.config = json.load(open(base_dir / "config.json"))
        self.cmd = cmd

    def base_port(self, i):
        return self.config["validators"][i]["base_port"]

    def node_rpc(self, i):
        return "tcp://127.0.0.1:%d" % ports.rpc_port(self.base_port(i))

    def cosmos_cli(self, i=0):
        return CosmosCLI(self.base_dir / f"node{i}", self.node_rpc(i), self.cmd)


class Cronos(Chain):
    def __init__(self, base_dir):
        Chain.__init__(self, base_dir, "cronosd")
        self._w3 = None
        self.enable_auto_deployment = json.load(open(base_dir / "genesis.json"))[
            "app_state"
        ]["cronos"]["params"]["enable_auto_deployment"]

    @property
    def w3(self, i=0):
        if self._w3 is None:
            port = ports.evmrpc_port(self.base_port(i))
            self._w3 = web3.Web3(
                web3.providers.HTTPProvider(f"http://localhost:{port}")
            )
        return self._w3


class Chainmain(Chain):
    def __init__(self, base_dir):
        Chain.__init__(self, base_dir, "chain-maind")


def setup_chainmain(path, base_port):
    cfg = Path(__file__).parent / "../scripts/chainmain-devnet.yaml"
    proc = setup_pystarport(path, base_port, cfg)
    try:
        wait_for_port(base_port)
        yield Chainmain(path / "chainmain-1")
    finally:
        os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
        # proc.terminate()
        proc.wait()


def setup_cronos(path, base_port, enable_auto_deployment=True):
    cfg = Path(__file__).parent / (
        "../scripts/cronos-devnet.yaml"
        if enable_auto_deployment
        else "configs/disable_auto_deployment.yaml"
    )
    proc = setup_pystarport(path, base_port, cfg)
    try:
        wait_for_port(ports.evmrpc_port(base_port))
        yield Cronos(path / "cronos_777-1")
    finally:
        os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
        # proc.terminate()
        proc.wait()


def setup_pystarport(path, base_port, config):
    cmd = [
        "pystarport",
        "serve",
        "--config",
        config,
        "--data",
        path,
        "--base_port",
        str(base_port),
        "--quiet",
    ]
    print(*cmd)
    return subprocess.Popen(
        cmd,
        preexec_fn=os.setsid,
    )


def wait_for_port(port, host="127.0.0.1", timeout=40.0):
    start_time = time.perf_counter()
    while True:
        try:
            with socket.create_connection((host, port), timeout=timeout):
                break
        except OSError as ex:
            time.sleep(0.1)
            if time.perf_counter() - start_time >= timeout:
                raise TimeoutError(
                    "Waited too long for the port {} on host {} to start accepting "
                    "connections.".format(port, host)
                ) from ex

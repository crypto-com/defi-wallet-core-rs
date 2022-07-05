#!/usr/bin/env python3

import json
import os
from pathlib import Path

from dotenv import load_dotenv
from eth_account import Account
from web3._utils.transactions import fill_nonce, fill_transaction_defaults

load_dotenv(Path(__file__).parent.parent / "scripts/.env")
Account.enable_unaudited_hdwallet_features()

ACCOUNTS = {
    "validator": Account.from_mnemonic(os.getenv("VALIDATOR1_MNEMONIC")),
    "community": Account.from_mnemonic(os.getenv("COMMUNITY_MNEMONIC")),
    "signer1": Account.from_mnemonic(os.getenv("SIGNER1_MNEMONIC")),
    "signer2": Account.from_mnemonic(os.getenv("SIGNER2_MNEMONIC")),
}
KEYS = {name: account.key for name, account in ACCOUNTS.items()}
ADDRS = {name: account.address for name, account in ACCOUNTS.items()}
TEST_CONTRACTS = {
    "TestERC20": "TestERC20.sol",
    "TestERC721": "TestERC721.sol",
    "TestERC1155": "TestERC1155.sol",
}

CPP_EXAMPLE_PATH = os.getenv("CPP_EXAMPLE_PATH")


def contract_path(name, filename):
    return (
        Path(__file__).parent.parent
        / "contracts/artifacts/contracts"
        / filename
        / (name + ".json")
    )


CONTRACTS = {
    "ModuleCRC20": Path(__file__).parent.parent
    / "x/cronos/types/contracts/ModuleCRC20.json",
    **{
        name: contract_path(name, filename) for name, filename in TEST_CONTRACTS.items()
    },
}


class Contract:
    "General contract."

    def __init__(self, contract_path, private_key=KEYS["validator"], chain_id=777):
        self.chain_id = chain_id
        self.account = Account.from_key(private_key)
        self.address = self.account.address
        self.private_key = private_key
        with open(contract_path) as f:
            json_data = f.read()
            contract_json = json.loads(json_data)
        self.bytecode = contract_json["bytecode"]
        self.abi = contract_json["abi"]
        self.contract = None
        self.w3 = None

    def sign(self, tx):
        "fill default fields and sign"
        acct = Account.from_key(self.private_key)
        tx["from"] = acct.address
        tx = fill_transaction_defaults(self.w3, tx)
        tx = fill_nonce(self.w3, tx)
        return acct.sign_transaction(tx)

    def send(self, tx):
        signed = self.sign(tx)
        txhash = self.w3.eth.send_raw_transaction(signed.rawTransaction)
        return self.w3.eth.wait_for_transaction_receipt(txhash)

    def deploy(self, w3, *args, **kwargs):
        "Deploy Greeter contract on `w3` and return the receipt."
        if self.contract is None:
            self.w3 = w3
            contract = self.w3.eth.contract(abi=self.abi, bytecode=self.bytecode)
            transaction = contract.constructor(*args, **kwargs).buildTransaction(
                {"chainId": self.chain_id, "from": self.address}
            )
            receipt = self.send(transaction)
            self.contract = self.w3.eth.contract(
                address=receipt.contractAddress, abi=self.abi
            )
            return self.contract
        else:
            return self.contract

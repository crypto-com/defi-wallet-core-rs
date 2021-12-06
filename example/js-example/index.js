import * as wasm from "@crypto-com/defi-wallet-core-common";

// FIXME: wallet keygen
const pubkey = new Uint8Array([3, 127, 12, 203, 79, 3, 211, 37, 157, 178, 152, 47, 245, 142, 167, 89, 207, 9, 218, 144, 20, 147, 186, 114, 170, 114, 137, 201, 226, 149, 141, 113, 14]);
const account_number = BigInt(1);
const sequence_number = BigInt(1);
const gas_limit = BigInt(100000);
const fee_amount = BigInt(1000000);
const fee_denom = "uatom";
const timeout_height = 9001;
const memo_note = "example memo";
const chain_id = "cosmoshub-4";
const bech32hrp = "cosmos";
const coin_type = 118;

const tx_info = wasm.new_sdk_tx_info(pubkey,
    account_number,
    sequence_number,
    gas_limit,
    fee_amount,
    fee_denom,
    timeout_height,
    memo_note,
    chain_id,
    bech32hrp,
    coin_type);
const tx_signdoc = wasm.get_single_bank_send_signdoc(tx_info,
    "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z",
    BigInt(1000000), "uatom");
console.log(tx_signdoc);

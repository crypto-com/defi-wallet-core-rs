import * as wasm from "@crypto-com/defi-wallet-core-wasm";

const wallet = new wasm.Wallet();
const cosmos_hub_address = wallet.get_default_address(wasm.CoinType.CosmosHub);
console.log(`Wallet Cosmos Hub address: ${cosmos_hub_address}`);
const eth_address = wallet.get_default_address(wasm.CoinType.Ethereum);
console.log(`Wallet Ethereum address: ${eth_address}`);

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

const tx_info = new wasm.CosmosSDKTxInfoRaw(account_number, sequence_number, gas_limit, fee_amount, fee_denom, timeout_height, memo_note, chain_id, bech32hrp, coin_type);

// const tx_signdoc = wasm.get_single_bank_send_signdoc(tx_info,
//     pubkey,
//     "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z",
//     BigInt(1000000), "uatom");
// console.log(tx_signdoc);

// constructs private key from bytes
const privateKey1 = wasm.PrivateKey.from_bytes([68, 130, 23, 78, 109, 255, 54, 116, 253, 157, 134, 231, 202, 245, 109, 197, 25, 56, 195, 182, 224, 75, 239, 191, 220, 164, 170, 198, 159, 113, 5, 255]);
logPrivateKeyInternal(privateKey1);

// constructs private key from hex
const privateKey2 = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
logPrivateKeyInternal(privateKey2);

// generates a random private key
const privateKey3 = new wasm.PrivateKey();
logPrivateKeyInternal(privateKey3);

const signed_tx = wasm.get_single_bank_send_signed_tx(tx_info, privateKey3, "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z",
BigInt(1000000), "uatom");
console.log(signed_tx);

const account = await wasm.query_account_details("https://testnet-croeseid-4.crypto.org:1317", "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy");
console.log(account);

const balance = await wasm.query_account_balance("https://testnet-croeseid-4.crypto.org:1317", "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy", "basetcro", 0);
console.log(balance);

const tx_resp = await wasm.broadcast_tx("https://testnet-croeseid-4.crypto.org:26657", signed_tx);
console.log(tx_resp);
const eth_balance = await wasm.query_account_eth_balance("https://cronos-testnet-3.crypto.org:8545", "0x2c600e0a72b3ae39e9b27d2e310b180abe779368", );
console.log(eth_balance);
const key2 = wallet.get_key("m/44'/60'/0'/0/0");
const receipt = await wasm.broadcast_transfer_eth("https://cronos-testnet-3.crypto.org:8545", "0x2c600e0a72b3ae39e9b27d2e310b180abe779368", "1.0", BigInt(338), key2);
console.log(receipt);

function logPrivateKeyInternal(privateKey) {
  const publicKeyBytes = privateKey.get_public_key_bytes();
  const publicKeyHex = privateKey.get_public_key_hex();
  const privateKeyBytes = privateKey.to_bytes();
  const privateKeyHex = privateKey.to_hex();

  console.log(
    "Private Key Internal",
    `\nPublic Key Bytes: ${publicKeyBytes}`,
    `\nPublic Key Hex: ${publicKeyHex}`,
    `\nPrivate Key Bytes: ${privateKeyBytes}`,
    `\nPrivate Key Hex: ${privateKeyHex}`
  );
}

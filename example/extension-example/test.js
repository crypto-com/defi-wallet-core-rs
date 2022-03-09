import init, * as wasm from "./node_modules/@crypto-com/defi-wallet-core-wasm/defi_wallet_core_wasm.js";

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

async function wallet_demo() {
  await init();

  var wallet = new wasm.Wallet();
  var address = wallet.get_default_address(wasm.CoinType.CosmosHub);
  console.log(address);

  wallet = new wasm.Wallet("",wasm.MnemonicWordCount.Twelve);
  var mnemonic = wallet.get_backup_mnemonic_phrase();
  console.log("mnemonic 12:",mnemonic);

  wallet = new wasm.Wallet("",wasm.MnemonicWordCount.Eighteen);
  var mnemonic = wallet.get_backup_mnemonic_phrase();
  console.log("mnemonic 18:",mnemonic);

  wallet = new wasm.Wallet("",wasm.MnemonicWordCount.TwentyFour);
  var mnemonic = wallet.get_backup_mnemonic_phrase();
  console.log("mnemonic 24:",mnemonic);

  let words = "guard input oyster oyster slot doctor repair shed soon assist blame power";
  wallet = wasm.Wallet.recover_wallet(words,"");
  mnemonic = wallet.get_backup_mnemonic_phrase();
  console.log("mnemonic:",mnemonic);
  console.assert(words === mnemonic);
  address = wallet.get_default_address(wasm.CoinType.CryptoOrgMainnet);
  console.assert(address === "cro16edxe89pn8ly9c7cy702x9e62fdvf3k9tnzycj");

  address = wallet.get_address(wasm.CoinType.CryptoOrgMainnet,1);
  console.assert(address === "cro1keycl6d55fnlzwgfdufl53vuf95uvxnry6uj2q");

  var priv = wallet.get_key("m/44'/394'/0'/0/0");
  console.assert(priv.to_hex() === "2e9c6bc5d8df5177697e90e87bd098d2d6165f096195d78f76cca1cecbf37525");
  logPrivateKeyInternal(priv);
  
  priv = wasm.PrivateKey.from_hex("e7de4e2f72573cf3c6e1fa3845cec6a4e2aac582702cac14bb9da0bb05aa24ae");
  console.assert(priv.get_public_key_hex() === "03cefab3f89c62ecc54c09634516bb2819d20d83757956c7f4690dc3b806ecc7d2");
}

async function cosmos_demo() {
  await init();
  const WORDS = "apple elegant knife hawk there screen vehicle lounge tube sun engage bus custom market pioneer casual wink present cat metal ride shallow fork brief";
  var wallet = wasm.Wallet.recover_wallet(WORDS,"");

  var priv = wallet.get_key("m/44'/118'/0'/0/0");
  
  const pubkey = new Uint8Array([3, 127, 12, 203, 79, 3, 211, 37, 157, 178, 152, 47, 245, 142, 167, 89, 207, 9, 218, 144, 20, 147, 186, 114, 170, 114, 137, 201, 226, 149, 141, 113, 14]);
  const account_number = BigInt(1);
  const sequence_number = BigInt(0);
  const gas_limit = BigInt(100000);
  const fee_amount = BigInt(1000000);
  const fee_denom = "uatom";
  const timeout_height = 9001;
  const memo_note = null;
  const chain_id = "cosmoshub-4";
  const bech32hrp = "cosmos";
  const coin_type = 118;
  
  const tx_info = new wasm.CosmosSDKTxInfoRaw(account_number, sequence_number, gas_limit, fee_amount, fee_denom, timeout_height, memo_note, chain_id, bech32hrp, coin_type);
  var tx_raw = wasm.get_single_bank_send_signdoc(tx_info, priv.get_public_key_bytes(), "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z", BigInt(1000000), "uatom");
  console.log("tx_raw:",wasm.bytes2hex(tx_raw));
  
}

wallet_demo();
cosmos_demo();

import * as wasm from "@crypto-com/defi-wallet-core-wasm";

// Constants

const CHAIN_ID = "chainmain-1";
const CHAINMAIN_DENOM = "basecro";
const CHAINMAIN_API_URL = "http://127.0.0.1:26804";
const TENDERMINT_RPC_URL = "http://127.0.0.1:26807";
const DELEGATOR2 = "cro1tmfhgwp62uhz5y5hqcyl8jkjq22l2cles2lum8";
const VALIDATOR1 = "crocncl1pk9eajj4zuzpptnadwz6tzfgcpchqvpkvql0a9";
const DEFAULT_GAS_LIMIT = BigInt(50_000_000);
const DEFAULT_FEE_AMOUNT = BigInt(25_000_000_000);
const BANK_SEND_AMOUNT = BigInt(50_000_000_000);
const STAKING_DELEGATE_AMOUNT = BigInt(100_000_000_000);

// Main workflow

testPrivateKey();
testCosmosSignDirect();
testEthSign();
testEip199PersonalSign();
testEip712TypedDataSign();
testBuildEthereumContractBatchTransfer();
const txData = testBuildAndSignCosmosTx();
testCosmosClient(txData);

const wallet = new wasm.Wallet();
logWalletAddresses(wallet);

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
const signed_tx = wasm.get_single_bank_send_signed_tx(tx_info, new wasm.PrivateKey(), "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z",
BigInt(1000000), "uatom");
console.log(signed_tx);

// const account = await wasm.query_account_details("https://testnet-croeseid-4.crypto.org:1317", "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy");
// console.log(account);

// const balance = await wasm.query_account_balance("https://testnet-croeseid-4.crypto.org:1317", "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy", "basetcro", 0);
// console.log(balance);

// const tx_resp = await wasm.broadcast_tx("https://testnet-croeseid-4.crypto.org:26657", signed_tx);
// console.log(tx_resp);
// const eth_balance = await wasm.query_account_eth_balance("https://cronos-testnet-3.crypto.org:8545", "0x2c600e0a72b3ae39e9b27d2e310b180abe779368", );
// console.log(eth_balance);
// const key2 = wallet.get_key("m/44'/60'/0'/0/0");
// const receipt = await wasm.broadcast_transfer_eth("https://cronos-testnet-3.crypto.org:8545", "0x2c600e0a72b3ae39e9b27d2e310b180abe779368", "1.0", BigInt(338), key2);
// console.log(receipt);

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

function logWalletAddresses(wallet) {
  const cosmos_hub_address = wallet.get_default_address(wasm.CoinType.CosmosHub);
  console.log(`Wallet Cosmos Hub address: ${cosmos_hub_address}`);
  const eth_address = wallet.get_default_address(wasm.CoinType.Ethereum);
  console.log(`Wallet Ethereum address: ${eth_address}`);
}

function testCosmosSignDirect() {
  const authInfoBytes = "0a0a0a0012040a020801180112130a0d0a0575636f736d12043230303010c09a0c";
  const bodyBytes = "0a90010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e6412700a2d636f736d6f7331706b707472653766646b6c366766727a6c65736a6a766878686c63337234676d6d6b38727336122d636f736d6f7331717970717870713971637273737a673270767871367273307a716733797963356c7a763778751a100a0575636f736d120731323334353637";

  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signature = wasm.cosmos_signDirect(privateKey, "chaintest", "1", authInfoBytes, bodyBytes);
  console.log(`Cosmos signDirect signature: ${signature}`);
}

function testEthSign() {
  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signature = wasm.eth_sign(privateKey, "01020304050607085152535455565758a1a2a3a4a5a6a7a8f1f2f3f4f5f6f7f8");
  console.log(`eth_sign signature: ${signature}`);
}
function testEip199PersonalSign() {
  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signature = wasm.personal_sign(privateKey, "0xdeadbeaf");
  console.log(`EIP-199 personal_sign signature: ${signature}`);
}

function testEip712TypedDataSign() {
  const params = {
    types: {
      EIP712Domain: [
        { name: 'name', type: 'string' },
        { name: 'version', type: 'string' },
        { name: 'chainId', type: 'uint256' },
        { name: 'verifyingContract', type: 'address' },
      ],
      Person: [
        { name: 'name', type: 'string' },
        { name: 'wallet', type: 'address' },
      ],
    },
    primaryType: 'Person',
    domain: {
      name: 'Ether Person',
      version: '1',
      chainId: 1,
      verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
    },
    message: {
      name: 'Bob',
      wallet: '0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB',
    },
  };

  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signature = wasm.eth_signTypedData(privateKey, JSON.stringify(params));
  console.log(`EIP-712 eth_signTypedData signature: ${signature}`);
}

function testPrivateKey() {
  // Construct private key from bytes.
  const privateKey1 = wasm.PrivateKey.from_bytes([68, 130, 23, 78, 109, 255, 54, 116, 253, 157, 134, 231, 202, 245, 109, 197, 25, 56, 195, 182, 224, 75, 239, 191, 220, 164, 170, 198, 159, 113, 5, 255]);
  logPrivateKeyInternal(privateKey1);

  // Construct private key from hex.
  const privateKey2 = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  logPrivateKeyInternal(privateKey2);

  // Generate a random private key.
  const privateKey3 = new wasm.PrivateKey();
  logPrivateKeyInternal(privateKey3);
}

function testBuildAndSignCosmosTx() {
  // Get private key.
  const privateKey = new wasm.PrivateKey();

  // Construct transaction info.
  const txInfo = new wasm.CosmosSDKTxInfoRaw(
    BigInt(1),
    BigInt(1),
    DEFAULT_GAS_LIMIT,
    DEFAULT_FEE_AMOUNT,
    CHAINMAIN_DENOM,
    0,
    "example memo",
    CHAIN_ID,
    "cosmos",
    118,
  );

  // Create a transaction.
  const tx = new wasm.CosmosTx();

  // Add a staking delegate message.
  tx.add_msg(wasm.CosmosMsg.build_staking_delegate_msg(
      VALIDATOR1,
      STAKING_DELEGATE_AMOUNT,
      CHAINMAIN_DENOM,
  ));

  // Add a bank send message.
  tx.add_msg(wasm.CosmosMsg.build_bank_send_msg(
      DELEGATOR2,
      BANK_SEND_AMOUNT,
      CHAINMAIN_DENOM,
  ));

  // Sign the transaction and move out all pending messages.
  console.assert(tx.get_msg_count() === 2, "No message has been added to Cosmos transaction");
  const txData = tx.sign_into(privateKey, txInfo);
  console.assert(tx.get_msg_count() === 0, "Pending messages of Cosmos transaction have not been moved out");

  console.log(`Signed Cosmos transaction data: ${txData}`);
  return txData;
}

function testBuildEthereumContractBatchTransfer() {
  const details = wasm.ContractBatchTransferDetails.build_erc1155_safe_batch_transfer_from(
    "0x6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0",
    "0xcd234a471b72ba2f1ccf0a70fcaba648a5eecd8d",
    "0x343c43a37d37dff08ae8c4a11544c718abb4fcf8",
    // Array of token ID and amount of hex value pair
    [
      new wasm.TokenAmount("0x1344ead983", "0x6d22"),
      new wasm.TokenAmount("0x2b40d6d551", "0x8aaa"),
    ],
    // Additional data
    [1, 2, 3]
  );
  console.dir(details);
}

async function testCosmosClient(txData) {
  const config = new wasm.CosmosClientConfig(
    CHAINMAIN_API_URL,
    TENDERMINT_RPC_URL,
  );
  const client = new wasm.CosmosClient(config);
  await client.broadcast_tx(txData);
}

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
testCosmosProtoSigning();
testEip712TypedDataSimpleSigning();
testEip712TypedDataRecursivelyNestedSigning();
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

function testCosmosProtoSigning() {
  const bodyBytes = [
    10, 156, 1, 10, 37, 47, 99, 111, 115, 109, 111, 115, 46, 115, 116, 97, 107, 105, 110,
    103, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 85, 110, 100, 101, 108, 101,
    103, 97, 116, 101, 18, 115, 10, 45, 99, 111, 115, 109, 111, 115, 49, 108, 53, 115, 55,
    116, 110, 106, 50, 56, 97, 55, 122, 120, 101, 101, 99, 107, 104, 103, 119, 108, 104,
    106, 121, 115, 56, 100, 108, 114, 114, 101, 102, 103, 113, 114, 52, 112, 106, 18, 52,
    99, 111, 115, 109, 111, 115, 118, 97, 108, 111, 112, 101, 114, 49, 57, 100, 121, 108,
    48, 117, 121, 122, 101, 115, 52, 107, 50, 51, 108, 115, 99, 108, 97, 48, 50, 110, 48,
    54, 102, 99, 50, 50, 104, 52, 117, 113, 52, 101, 54, 52, 107, 51, 26, 12, 10, 5, 117,
    97, 116, 111, 109, 18, 3, 49, 48, 48, 24, 169, 70,
  ];

  const authInfoBytes = [
    10, 78, 10, 70, 10, 31, 47, 99, 111, 115, 109, 111, 115, 46, 99, 114, 121, 112, 116,
    111, 46, 115, 101, 99, 112, 50, 53, 54, 107, 49, 46, 80, 117, 98, 75, 101, 121, 18, 35,
    10, 33, 2, 140, 57, 86, 222, 0, 17, 214, 185, 178, 199, 53, 4, 86, 71, 209, 75, 56,
    230, 53, 87, 228, 151, 252, 2, 93, 233, 161, 122, 87, 41, 197, 32, 18, 4, 10, 2, 8, 1,
    18, 22, 10, 16, 10, 5, 117, 97, 116, 111, 109, 18, 7, 49, 48, 48, 48, 48, 48, 48, 16,
    160, 141, 6,
  ];

  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signer = new wasm.CosmosSigner(privateKey);
  const signedData = signer.sign_direct(bodyBytes, authInfoBytes, "chaintest", BigInt(1));
  console.log(`Cosmos signDirect data: ${signedData}`);
}

function testEip712TypedDataRecursivelyNestedSigning() {
  const params = {
    types: {
      EIP712Domain: [
        { name: 'name', type: 'string' },
        { name: 'version', type: 'string' },
        { name: 'chainId', type: 'uint256' },
        { name: 'verifyingContract', type: 'address' },
      ],
      Mail: [
        { name: 'from', type: 'Person' },
        { name: 'to', type: 'Person' },
        { name: 'contents', type: 'string' },
      ],
      Person: [
        { name: 'name', type: 'string' },
        { name: 'wallet', type: 'address' },
      ],
    },
    primaryType: 'Mail',
    domain: {
      name: 'Ether Mail',
      version: '1',
      chainId: 1,
      verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
    },
    message: {
      from: {
        name: 'Cow',
        wallet: '0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826',
      },
      to: {
        name: 'Bob',
        wallet: '0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB',
      },
      contents: 'Hello, Bob!',
    },
  };

  const privateKey = wasm.PrivateKey.from_hex("af6f293f2621bfb5a70d7cf123596bd14827f73769c24edf2688b3ce2c86d747");
  const signer = new wasm.EthSigner(privateKey);
  const signedData = signer.sign_typed_data(JSON.stringify(params));
  console.log(`EIP-712 recursively nested signTypedData: ${signedData}`);
}

function testEip712TypedDataSimpleSigning() {
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
  const signer = new wasm.EthSigner(privateKey);
  const signedData = signer.sign_typed_data(JSON.stringify(params));
  console.log(`EIP-712 simple signTypedData: ${signedData}`);
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

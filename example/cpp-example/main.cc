#include "defi-wallet-core-cpp/src/lib.rs.h"
#include "defi-wallet-core-cpp/src/nft.rs.h"
#include "rust/cxx.h"
#include <cassert>
#include <chrono>
#include <iostream>
#include <thread>

void cronos_process();
using namespace std;
using namespace org::defi_wallet_core;
using namespace std::this_thread; // sleep_for, sleep_until
using namespace std::chrono;      // nanoseconds, system_clock, seconds
CosmosSDKTxInfoRaw build_txinfo() {
  CosmosSDKTxInfoRaw ret;
  ret.account_number = 0;
  ret.sequence_number = 0;
  ret.gas_limit = 5000000;
  ret.fee_amount = 25000000000;
  ret.fee_denom = "basecro";
  ret.timeout_height = 0;
  ret.memo_note = "";
  ret.chain_id = "";
  ret.coin_type = 394;
  ret.bech32hrp = "cro";
  return ret;
}
string getEnv(string key) {
  string ret;
  if (getenv(key.c_str()) != nullptr) {
    ret = getenv(key.c_str());
  }
  return ret;
}
rust::cxxbridge1::Box<Wallet>
createWallet(rust::cxxbridge1::String mymnemonics) {

  try {
    rust::cxxbridge1::Box<Wallet> mywallet = restore_wallet(mymnemonics, "");
    return mywallet;
  } catch (const rust::cxxbridge1::Error &e) {
    cout << "invalid mnemonics" << endl;
    cout << "export MYMNEMONICS=<your mnemonics>" << endl;
    throw e;
  }
}

void test_chainmain_nft() {
  using namespace rust::cxxbridge1;

  CosmosSDKTxInfoRaw tx_info = build_txinfo();

  String myservertendermint = getEnv("MYTENDERMINTRPC");
  String mygrpc = getEnv("MYGRPC");
  String myservercosmos = getEnv("MYCOSMOSRPC");

  String myfrom = getEnv("MYFROM");
  String myto = getEnv("MYTO");
  String mychainid = getEnv("MYCHAINID");

  String signer1_mnemonics = getEnv("SIGNER1_MNEMONIC");
  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  char hdpath[100];
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", tx_info.coin_type);
  Box<PrivateKey> signer1_private_key =
      createWallet(signer1_mnemonics)->get_key(hdpath);
  Box<PrivateKey> signer2_private_key =
      createWallet(signer2_mnemonics)->get_key(hdpath);

  CosmosAccountInfoRaw detailinfo =
      query_account_details_info(myservercosmos, myfrom);
  auto signer1_sn = detailinfo.sequence_number;
  auto signer1_ac = detailinfo.account_number;

  detailinfo = query_account_details_info(myservercosmos, myto);
  auto signer2_sn = detailinfo.sequence_number;
  auto signer2_ac = detailinfo.account_number;

  tx_info.chain_id = mychainid;
  tx_info.account_number = signer1_ac;
  tx_info.sequence_number = signer1_sn;

  // chainmain nft tests
  auto denom_id = "testdenomid";
  auto denom_name = "testdenomname";
  auto schema = R""""(
  {
    "title": "Asset Metadata",
    "type": "object",
    "properties": {
      "name": {
        "type": "string",
        "description": "testidentity"
      },
      "description": {
        "type": "string",
        "description": "testdescription"
      },
      "image": {
        "type": "string",
        "description": "testdescription"
      }
    }
  })"""";

  // issue: myfrom
  signer1_sn += 1;
  tx_info.sequence_number = signer1_sn;
  Vec<uint8_t> signedtx = get_nft_issue_denom_signed_tx(
      tx_info, *signer1_private_key, denom_id, denom_name, schema);

  String resp = broadcast_tx(myservertendermint, signedtx);
  cout << "issue response: " << resp << endl;

  auto token_id = "testtokenid";
  auto token_name = "testtokenname";
  auto token_uri = "testtokenuri";
  auto token_data = "";

  // mint: myfrom -> myto
  signer1_sn += 1;
  tx_info.sequence_number = signer1_sn;
  signedtx =
      get_nft_mint_signed_tx(tx_info, *signer1_private_key, token_id, denom_id,
                             token_name, token_uri, token_data, myto);
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "mint response: " << resp << endl;

  sleep_for(seconds(3));
  Box<GrpcClient> grpc_client = new_grpc_client(mygrpc);

  Vec<Denom> denoms = grpc_client->denoms();
  assert(denoms.size() == 1);
  assert(denoms[0].id == denom_id);
  assert(denoms[0].name == denom_name);
  assert(denoms[0].schema == schema);
  assert(denoms[0].creator == myfrom);

  BaseNft nft = grpc_client->nft(denom_id, token_id);
  cout << "nft: " << nft.to_string() << endl;
  assert(nft.id == token_id);
  assert(nft.name == token_name);
  assert(nft.uri == token_uri);
  assert(nft.data == token_data);
  assert(nft.owner == myto);

  Collection collection = grpc_client->collection(denom_id);
  cout << "collection: " << collection.to_string() << endl;
  Owner owner = grpc_client->owner(denom_id, myto);
  cout << "owner: " << owner.to_string() << endl;
  assert(owner.address == myto);
  assert(owner.id_collections.size() == 1);
  assert(owner.id_collections[0].denom_id == denom_id);
  assert(owner.id_collections[0].token_ids.size() == 1);
  assert(owner.id_collections[0].token_ids[0] == token_id);

  // transfer: myto -> myfrom
  tx_info.account_number = signer2_ac;
  tx_info.sequence_number = signer2_sn;
  signedtx = get_nft_transfer_signed_tx(tx_info, *signer2_private_key, token_id,
                                        denom_id, myfrom);
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "transfer response: " << resp << endl;
  sleep_for(seconds(3));
  nft = grpc_client->nft(denom_id, token_id);
  cout << "nft: " << nft.to_string() << endl;
  assert(nft.id == token_id);
  assert(nft.name == token_name);
  assert(nft.uri == token_uri);
  assert(nft.data == token_data);
  assert(nft.owner == myfrom);
  owner = grpc_client->owner(denom_id, myfrom);
  cout << "owner: " << owner.to_string() << endl;
  assert(owner.address == myfrom);
  assert(owner.id_collections.size() == 1);
  assert(owner.id_collections[0].denom_id == denom_id);
  assert(owner.id_collections[0].token_ids.size() == 1);
  assert(owner.id_collections[0].token_ids[0] == token_id);

  // edit
  tx_info.account_number = signer1_ac;
  signer1_sn += 1;
  tx_info.sequence_number = signer1_sn;
  signedtx = get_nft_edit_signed_tx(tx_info, *signer1_private_key, token_id,
                                    denom_id, "newname", "newuri", "newdata");
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "edit response: " << resp << endl;
  sleep_for(seconds(3));
  nft = grpc_client->nft(denom_id, token_id);
  cout << "nft: " << nft.to_string() << endl;
  assert(nft.id == token_id);
  assert(nft.name == "newname");
  assert(nft.uri == "newuri");
  assert(nft.data == "newdata");
  assert(nft.owner == myfrom);
  int supply = grpc_client->supply(denom_id, myfrom);
  cout << "supply: " << supply << endl;
  assert(supply == 1);

  // burn
  signer1_sn += 1;
  tx_info.sequence_number = signer1_sn;
  signedtx =
      get_nft_burn_signed_tx(tx_info, *signer1_private_key, token_id, denom_id);
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "burn response: " << resp << endl;
  sleep_for(seconds(3));
  supply = grpc_client->supply(denom_id, myfrom);
  cout << "supply: " << supply << endl;
  assert(supply == 0);
}

void process() {
  cout << "defi-wallet-core-rs cpp-example" << endl;
  CosmosSDKTxInfoRaw tx_info = build_txinfo();

  cout << "declare these environment variables:" << endl;
  cout << "export MYMNEMONICS=\"your mnemonics\"" << endl;
  cout << "export MYCOSMOSRPC=\"http://yourcosmosnode:1317\"" << endl;
  cout << "export MYTENDERMINTRPC=\"http://yourcosmosnode:26657\"" << endl;
  cout << "export MYGRPC=\"http://yourcosmosnode:9091\"" << endl;
  cout << "export MYCHAINID=your-chainid-1" << endl;
  cout << "export MYFROM=cro1yourwalletaddress" << endl;
  cout << "export MYTO=cro1yourreceiveraddress" << endl;
  cout << "------------------------------------------------------" << endl;

  rust::cxxbridge1::String mymnemonics = getEnv("MYMNEMONICS");
  string mychainid = getEnv("MYCHAINID");
  string myfrom = getEnv("MYFROM");
  string myto = getEnv("MYTO");
  string myamount = getEnv("MYAMOUNT");
  string myservercosmos = getEnv("MYCOSMOSRPC");         /* 1317 port */
  string myservertendermint = getEnv("MYTENDERMINTRPC"); /* 26657 port */
  string mygrpc = getEnv("MYGRPC");                      /* 9091 port */
  rust::cxxbridge1::Box<Wallet> mywallet = createWallet(mymnemonics);
  cout << "transfer from " << myfrom << " to " << myto << " amount " << myamount
       << endl;
  rust::cxxbridge1::String success, fail;
  rust::cxxbridge1::String result =
      mywallet->get_default_address(CoinType::CryptoOrgMainnet);
  rust::cxxbridge1::String balance =
      query_account_balance(myservercosmos, myfrom, tx_info.fee_denom, 1);
  cout << "balance=" << balance.c_str() << endl;
  rust::cxxbridge1::String detailjson =
      query_account_details(myservercosmos, myfrom);
  cout << "detailjson=" << detailjson.c_str() << endl;
  CosmosAccountInfoRaw detailinfo =
      query_account_details_info(myservercosmos, myfrom);
  tx_info.account_number = detailinfo.account_number;
  tx_info.sequence_number = detailinfo.sequence_number;
  tx_info.chain_id = mychainid;
  char hdpath[100];
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", tx_info.coin_type);
  rust::cxxbridge1::Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  rust::cxxbridge1::Vec<uint8_t> signedtx =
      get_single_bank_send_signed_tx(tx_info, *privatekey, myto, 1, "basecro");
  rust::cxxbridge1::String resp = broadcast_tx(myservertendermint, signedtx);
}

void test_login() {
  cout << "testing login" << endl;

  // no \n in end of string
  std::string info =
      "service.org wants you to sign in with your Ethereum account:\n"
      "0xD09F7C8C4529CB5D387AA17E33D707C529A6F694\n"
      "\n"
      "I accept the ServiceOrg Terms of Service: https://service.org/tos\n"
      "\n"
      "URI: https://service.org/login\n"
      "Version: 1\n"
      "Chain ID: 1\n"
      "Nonce: 32891756\n"
      "Issued At: 2021-09-30T16:25:24Z\n"
      "Resources:\n"
      "- ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/\n"
      "- https://example.com/my-web2-claim.json";
  rust::cxxbridge1::Box<CppLoginInfo> logininfo = new_logininfo(info);

  rust::cxxbridge1::String mymnemonics = getEnv("MYMNEMONICS");
  rust::cxxbridge1::Box<Wallet> mywallet = createWallet(mymnemonics);

  char hdpath[100];
  int coin_type = 60; // eth cointype
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", coin_type);
  rust::cxxbridge1::Box<PrivateKey> privatekey = mywallet->get_key(hdpath);

  rust::cxxbridge1::String default_address =
      mywallet->get_default_address(CoinType::CronosMainnet);
  rust::cxxbridge1::Vec<uint8_t> signature =
      logininfo->sign_logininfo(*privatekey);
  assert(signature.size() == 65);
  rust::Slice<const uint8_t> slice{signature.data(), signature.size()};
  logininfo->verify_logininfo(slice);
}

int main() {
  try {
    process();            // chain-main
    test_chainmain_nft(); // chainmain nft tests
    test_login();         // decentralized login
    cronos_process();     // cronos
  } catch (const rust::cxxbridge1::Error &e) {
    cout << "error:" << e.what() << endl;
  }
  return 0;
}

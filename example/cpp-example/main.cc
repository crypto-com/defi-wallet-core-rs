
#include "cxx.h"
#include "lib.rs.h"
#include <cassert>
#include <chrono>
#include <iostream>
#include <thread>

#include "nft.rs.h"
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

void test_chainmain_nft(CosmosSDKTxInfoRaw tx_info, PrivateKey &privatekey,
                        string myfrom, string myto) {

  string myservertendermint = getEnv("MYTENDERMINTRPC");
  string mygrpc = getEnv("MYGRPC");

  // chainmain nft tests
  tx_info.sequence_number += 1;
  auto denom_id = "testdenomid";
  auto denom_name = "testdenomname";
  auto schema = R""""(
                                {
                                "title":"Asset Metadata",
                                "type":"object",
                                "properties":{
                                "name":{
                                "type":"string",
                                "description":"testidentity"
                              },
                                "description":{
                                "type":"string",
                                "description":"testdescription"
                              },
                                "image":{
                                "type":"string",
                                "description":"testdescription"
                              }
                              }
                              })"""";

  rust::cxxbridge1::Vec<uint8_t> signedtx = get_nft_issue_denom_signed_tx(
      tx_info, privatekey, denom_id, denom_name, schema);

  rust::cxxbridge1::String resp = broadcast_tx(myservertendermint, signedtx);
  cout << "issue response: " << resp << endl;

  tx_info.sequence_number += 1;
  signedtx = get_nft_mint_signed_tx(tx_info, privatekey, "testtokenid",
                                    "testdenomid", "", "testuri", "", myto);
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "mint response: " << resp << endl;

  sleep_for(seconds(3));
  rust::cxxbridge1::Box<GrpcClient> grpc_client = new_grpc_client(mygrpc);

  rust::cxxbridge1::Vec<Denom> denoms = grpc_client->denoms();
  assert(denoms.size() == 1);
  assert(denoms[0].id == "testdenomid");
  assert(denoms[0].name == "testdenomname");
  assert(denoms[0].creator == myfrom);

  BaseNft nft =
      grpc_client->nft("testdenomid", "testtokenid");
  cout << "nft: " << nft.to_string() << endl;
  Collection collection =
      grpc_client->collection("testdenomid");
  cout << "collection: " << collection.to_string() << endl;
  Owner owner = grpc_client->owner("testdenomid", myto);
  cout << "owner: " << owner.to_string() << endl;

  tx_info.sequence_number += 1;
  signedtx = get_nft_transfer_signed_tx(tx_info, privatekey, "testtokenid",
                                        "testdenomid", myfrom);
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "transfer response: " << resp << endl;
  nft = grpc_client->nft("testdenomid", "testtokenid");
  cout << "nft: " << nft.to_string() << endl;
  owner = grpc_client->owner("testdenomid", myto);
  cout << "owner: " << owner.to_string() << endl;

  tx_info.sequence_number += 1;
  signedtx =
      get_nft_edit_signed_tx(tx_info, privatekey, "testtokenid", "testdenomid",
                             "newname", "newuri", "newdata");
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "edit response: " << resp << endl;
  nft = grpc_client->nft("testdenomid", "testtokenid");
  cout << "nft: " << nft.to_string() << endl;
  int supply = grpc_client->supply("testdenomid", myto);
  cout << "supply: " << supply << endl;

  tx_info.sequence_number += 1;
  signedtx =
      get_nft_burn_signed_tx(tx_info, privatekey, "testtokenid", "testdenomid");
  resp = broadcast_tx(myservertendermint, signedtx);
  cout << "burn response: " << resp << endl;
  collection =
      grpc_client->collection("testdenomid");
  cout << "collection: " << collection.to_string() << endl;
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

  // chainmain nft tests
  test_chainmain_nft(tx_info, *privatekey, myfrom, myto);

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
    process();        // chain-main
    test_login();     // decentralized login
    cronos_process(); // cronos
  } catch (const rust::cxxbridge1::Error &e) {
    cout << "error:" << e.what() << endl;
  }
  return 0;
}

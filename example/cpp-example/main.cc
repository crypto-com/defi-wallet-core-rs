
#include "cxx.h"
#include "lib.rs.h"
#include <cassert>
#include <iostream>
using namespace std;
using namespace org::defi_wallet_core;
CosmosSDKTxInfoRaw build_txinfo() {
  CosmosSDKTxInfoRaw ret;
  ret.account_number = 0;
  ret.sequence_number = 0;
  ret.gas_limit = 100000;
  ret.fee_amount = 1000000;
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
  string mygrpc = getEnv("MYGRPC"); /* 9091 port */
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
  broadcast_tx(myservertendermint, signedtx);

  rust::cxxbridge1::Vec<DenomRaw> denoms = query_denoms(mygrpc);
  cout << denoms.size() << endl;
}
int main() {
  try {
    process();
  } catch (const rust::cxxbridge1::Error &e) {
    cout << "error:" << e.what() << endl;
  }
  return 0;
}

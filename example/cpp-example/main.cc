
#include "lib.rs.h"
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
void process() {
  cout << "defi-wallet-core-rs cpp-example" << endl;
  CosmosSDKTxInfoRaw tx_info = build_txinfo();
  rust::cxxbridge1::String  mymnemonics = getEnv("MYMNEMONICS");
  string  mychainid = getEnv("MYCHAINID");
  string myfrom = getEnv("MYFROM");
  string myto = getEnv("MYTO");
  string myamount = getEnv("MYAMOUNT");
  string myservercosmos = getEnv("MYSERVER");      /* 1317 port */
  string myservertendermint = getEnv("MYSERVER2"); /* 26657 port */
  rust::cxxbridge1::Box<org::defi_wallet_core::Wallet> mywallet =
      restore_wallet(mymnemonics, "");
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
  org::defi_wallet_core::CosmosAccountInfoRaw detailinfo =
      query_account_details_info(myservercosmos, myfrom);
  tx_info.account_number = detailinfo.account_number;
  tx_info.sequence_number = detailinfo.sequence_number;
  tx_info.chain_id = mychainid;
  char hdpath[100];
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", tx_info.coin_type);
  rust::cxxbridge1::Box<org::defi_wallet_core::PrivateKey> privatekey =
      mywallet->get_key(hdpath);
  rust::cxxbridge1::Vec<uint8_t> signedtx =
      get_single_bank_send_signed_tx(tx_info, *privatekey, myto, 1, "basecro");
  broadcast_tx(myservertendermint, signedtx);
}
int main() {
  try {
    process();
  } catch (const std::exception &e) {
    cout << "error:" << e.what() << endl;
  }
  return 0;
}

#include "cxx.h"
#include "lib.rs.h"
#include <iostream>

void cronos_process();
using namespace std;
using namespace org::defi_wallet_core;

std::string getEnv(std::string key);

rust::cxxbridge1::Box<Wallet>
createWallet(rust::cxxbridge1::String mymnemonics);

void cronos_process() {
  std::cout << "cronos process" << std::endl;
  rust::cxxbridge1::String mymnemonics = getEnv("MYMNEMONICS");
  rust::cxxbridge1::String mycronosrpc = getEnv("MYCRONOSRPC");
  rust::cxxbridge1::Box<Wallet> mywallet = createWallet(mymnemonics);
  rust::cxxbridge1::String myaddress1 = mywallet->get_eth_address(0);
  rust::cxxbridge1::String myaddress2 = mywallet->get_eth_address(1);
  auto nonce1 = get_eth_nonce(myaddress1.c_str(), mycronosrpc);
  char hdpath[100];
  int cointype = 60;
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);
  rust::cxxbridge1::Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  rust::cxxbridge1::Vec<uint8_t> data;
  org::defi_wallet_core::CronosTxInfoRaw eth_tx_info =   new_eth_tx_info();
  cout << myaddress2 << endl;
  eth_tx_info.to_address = myaddress2.c_str();
  eth_tx_info.nonce = nonce1;
  eth_tx_info.amount = "1";
  eth_tx_info.amount_unit = org::defi_wallet_core::EthAmount::EthDecimal;
  rust::Vec<::std::uint8_t> signedtx =
      build_eth_signed_tx(eth_tx_info, EthNetwork::Custom, 9000, *privatekey);
  rust::cxxbridge1::String balance =
      get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance << endl;
  rust::cxxbridge1::String txhash =
      broadcast_eth_signed_raw_tx(signedtx, mycronosrpc);
  cout << "txhash=" << txhash << endl;
  balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance << endl;
}
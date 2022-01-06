#include "lib.rs.h"
#include <iostream>
using namespace std;
using namespace org::defi_wallet_core;
int main() {
  cout << "defi-wallet-core-rs cpp-example" << endl;
  auto a = new_wallet("password");
  rust::cxxbridge1::String success, fail;
  auto result = a->get_default_address(CoinType::CryptoOrgTestnet);
  cout << "success=" << result.c_str() << endl;

  return 0;
}
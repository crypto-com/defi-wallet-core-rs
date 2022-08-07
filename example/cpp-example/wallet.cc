#include "sdk/include/defi-wallet-core-cpp/src/contract.rs.h"
#include "sdk/include/defi-wallet-core-cpp/src/lib.rs.h"
#include "sdk/include/defi-wallet-core-cpp/src/uint.rs.h"
#include "sdk/include/rust/cxx.h"
#include <cassert>
#include <chrono>
#include <cstring>
#include <iostream>

using namespace std;
using namespace org::defi_wallet_core;
using namespace rust;
String getEnv(String key);
void test_wallet_restore(String password) {
  rust::String mymnemonics = getEnv("SIGNER1_MNEMONIC");
  int index = 0;
  Box<Wallet> mywallet = restore_wallet(mymnemonics, "");
  rust::String backupmnemonics = mywallet->get_backup_mnemonic_phrase();
  assert(mymnemonics == backupmnemonics);
}

void test_wallet_generatemnemonics(String passowrd) {
  rust::String mymnemonics =
      generate_mnemonics("", MnemonicWordCount::TwentyFour);
  int index = 0;
  Box<Wallet> mywallet = restore_wallet(mymnemonics, "");
  rust::String backupmnemonics = mywallet->get_backup_mnemonic_phrase();
  assert(mymnemonics == backupmnemonics);
}
void test_wallet_new(String password) {
  Box<Wallet> mywallet = new_wallet("", MnemonicWordCount::Twelve);
  int index = 0;
  rust::String mymnemonics = mywallet->get_backup_mnemonic_phrase();

  Box<Wallet> mywallet2 = restore_wallet(mymnemonics, "");
  rust::String backupmnemonics = mywallet2->get_backup_mnemonic_phrase();
  assert(mymnemonics == backupmnemonics);
}
void test_wallet() {
  test_wallet_restore(String(""));
  test_wallet_generatemnemonics(String(""));
  test_wallet_new(String(""));
  test_wallet_restore(String("mypassword"));
  test_wallet_generatemnemonics(String("mypassword"));
  test_wallet_new(String("mypassword"));
}
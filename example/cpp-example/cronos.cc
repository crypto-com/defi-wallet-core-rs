#include "sdk/include/defi-wallet-core-cpp/src/contract.rs.h"
#include "sdk/include/defi-wallet-core-cpp/src/lib.rs.h"
#include "sdk/include/defi-wallet-core-cpp/src/uint.rs.h"
#include "sdk/include/defi-wallet-core-cpp/src/ethereum.rs.h"
#include "sdk/include/rust/cxx.h"
#include <cassert>
#include <chrono>
#include <cstring>
#include <iostream>
#include <sstream>
#include <fstream>
#include <iomanip>

using namespace std;
using namespace rust;
using namespace org::defi_wallet_core;

void test_uint();
void test_approval();

Box<Wallet> createWallet(String mymnemonics);
String getEnv(String key);

typedef std::chrono::time_point<std::chrono::high_resolution_clock> timepoint;
timepoint measure_time(timepoint t1, std::string label);

using namespace std;
using namespace org::defi_wallet_core;
using namespace rust;

String getEnv(String key);

Box<Wallet> createWallet(String mymnemonics);

void cronos_process() {
  // Start measuring time
  timepoint begin = std::chrono::high_resolution_clock::now();
  std::cout << "cronos process" << std::endl;
  String mymnemonics = getEnv("SIGNER1_MNEMONIC");
  String mycronosrpc = getEnv("MYCRONOSRPC");
  Box<Wallet> mywallet = createWallet(mymnemonics);
  begin = measure_time(begin, "createWallet");
  String myaddress1 = mywallet->get_eth_address(0);
  begin = measure_time(begin, "get_eth_address");
  String myaddress2 = mywallet->get_eth_address(1);
  begin = measure_time(begin, "get_eth_address");
  auto nonce1 = get_eth_nonce(myaddress1.c_str(), mycronosrpc);
  begin = measure_time(begin, "get_eth_nonce");
  char hdpath[100];
  int cointype = 60;
  int chainid = 777; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);
  Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  begin = measure_time(begin, "get_key");
  Vec<uint8_t> data;
  EthTxInfoRaw eth_tx_info = new_eth_tx_info();
  begin = measure_time(begin, "new_eth_tx_info");
  cout << myaddress2 << endl;
  eth_tx_info.to_address = myaddress2.c_str();
  eth_tx_info.nonce = nonce1;
  eth_tx_info.amount = "1";
  eth_tx_info.amount_unit = EthAmount::EthDecimal;
  begin = measure_time(begin, "new_eth_tx_info");
  Vec<uint8_t> signedtx =
      build_eth_signed_tx(eth_tx_info, chainid, true, *privatekey);
  begin = measure_time(begin, "build_eth_signed_tx");
  U256 balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  begin = measure_time(begin, "get_eth_balance");
  cout << "address=" << myaddress1.c_str() << " balance=" << balance.to_string()
       << endl;
  String status =
      broadcast_eth_signed_raw_tx(signedtx, mycronosrpc, 1000).status;
  begin = measure_time(begin, "broadcast_eth_signed_raw_tx");
  assert(status == "1");

  balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  begin = measure_time(begin, "get_eth_balance");
  cout << "address=" << myaddress1.c_str() << " balance=" << balance.to_string()
       << endl;

  Erc20 erc20 = new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A",
                          mycronosrpc, chainid)
                    .legacy();
  begin = measure_time(begin, "new_erc20");
  assert(erc20.name() == "Gold");
  begin = measure_time(begin, "erc20.name");
  assert(erc20.symbol() == "GLD");
  begin = measure_time(begin, "erc20.symbol");
  assert(erc20.decimals() == 18);
  begin = measure_time(begin, "erc20.decimals");
  U256 erc20_total_supply = erc20.total_supply();
  begin = measure_time(begin, "erc20.total_supply");
  assert(erc20_total_supply == u256("100000000000000000000000000"));
  U256 erc20_balance = erc20.balance_of(myaddress1);
  begin = measure_time(begin, "erc20.balance_of");
  assert(erc20_balance == erc20_total_supply);

  Erc721 erc721 = new_erc721("0x2305f3980715c9D247455504080b41072De38aB9",
                             mycronosrpc, chainid)
                      .legacy();
  begin = measure_time(begin, "new_erc721");
  assert(erc721.name() == "GameItem");
  begin = measure_time(begin, "erc721.name");
  assert(erc721.symbol() == "ITM");
  begin = measure_time(begin, "erc721.symbol");
  assert(erc721.token_uri("1") == "https://game.example/item-id-8u5h2m.json");
  begin = measure_time(begin, "erc721.token_uri");
  // cout << "Total Supply of ERC721=" << erc721.total_supply() << endl; // the
  // contract must support IERC721Enumerable
  assert(erc721.owner_of("1") == myaddress1);
  begin = measure_time(begin, "erc721.owner_of");
  assert(erc721.balance_of(myaddress1) == u256("1"));
  begin = measure_time(begin, "erc721.balance_of");

  Erc1155 erc1155 = new_erc1155("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc",
                                mycronosrpc, chainid)
                        .legacy();
  begin = measure_time(begin, "new_erc1155");
  // To be improved in the contract, now all uri are the same
  assert(erc1155.uri("0") == "https://game.example/api/item/{id}.json");
  begin = measure_time(begin, "erc1155.uri");
  assert(erc1155.uri("1") == "https://game.example/api/item/{id}.json");
  begin = measure_time(begin, "erc1155.uri");
  assert(erc1155.uri("2") == "https://game.example/api/item/{id}.json");
  begin = measure_time(begin, "erc1155.uri");
  assert(erc1155.uri("3") == "https://game.example/api/item/{id}.json");
  begin = measure_time(begin, "erc1155.uri");
  assert(erc1155.uri("4") == "https://game.example/api/item/{id}.json");
  begin = measure_time(begin, "erc1155.uri");
  assert(erc1155.balance_of(myaddress1, "0") == u256("1000000000000000000"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "1") ==
         u256("1000000000000000000000000000"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "2") == u256("1"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "3") == u256("1000000000"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "4") == u256("1000000000"));
  begin = measure_time(begin, "erc1155.balance_of");

  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  Box<Wallet> signer2_wallet = createWallet(signer2_mnemonics);
  begin = measure_time(begin, "createWallet");
  String signer2_address = signer2_wallet->get_eth_address(0);
  begin = measure_time(begin, "get_eth_address");
  Box<PrivateKey> signer2_privatekey = signer2_wallet->get_key(hdpath);
  begin = measure_time(begin, "get_key");

  // transfer erc20 token from signer1 to signer2
  status = erc20.transfer(signer2_address, "100", *privatekey).status;
  begin = measure_time(begin, "erc20.transfer");
  assert(status == "1");
  assert(erc20.balance_of(myaddress1) == erc20_balance.sub(u256("100")));
  begin = measure_time(begin, "erc20.balance_of");

  // transfer erc721 from signer1 to signer2
  status = erc721.transfer_from(myaddress1, signer2_address, "1", *privatekey)
               .status;
  begin = measure_time(begin, "erc20.transfer_from");
  assert(status == "1");
  assert(erc721.balance_of(myaddress1) == u256("0"));
  begin = measure_time(begin, "erc20.balance_of");
  assert(erc721.owner_of("1") == signer2_address);
  begin = measure_time(begin, "erc20.owner_of");

  // safe transfer erc721 from signer2 to signer1
  status = erc721
               .safe_transfer_from(signer2_address, myaddress1, "1",
                                   *signer2_privatekey)
               .status;
  begin = measure_time(begin, "erc721.safe_transfer_from");
  assert(status == "1");
  assert(erc721.balance_of(myaddress1) == u256("1"));
  begin = measure_time(begin, "erc721.balance_of");
  assert(erc721.owner_of("1") == myaddress1);
  begin = measure_time(begin, "erc721.owner_of");

  // safe transfer erc1155 from signer1 to signer2
  Vec<uint8_t> erc1155_data;
  status = erc1155.interval(3000)
               .safe_transfer_from(myaddress1, signer2_address, "0", "150",
                                   erc1155_data, *privatekey)
               .status;
  begin = measure_time(begin, "erc1155.safe_transfer_from");
  assert(status == "1");
  assert(erc1155.balance_of(myaddress1, "0") == u256("999999999999999850"));
  begin = measure_time(begin, "erc1155.balance_of");

  // safe batch transfer erc1155 from signer1 to signer2
  Vec<String> token_ids, amounts;
  token_ids.push_back("1");
  token_ids.push_back("2");
  token_ids.push_back("3");
  token_ids.push_back("4");

  amounts.push_back("200");
  amounts.push_back("1");
  amounts.push_back("300");
  amounts.push_back("400");
  status = erc1155
               .safe_batch_transfer_from(myaddress1, signer2_address, token_ids,
                                         amounts, erc1155_data, *privatekey)
               .status;
  begin = measure_time(begin, "erc1155.safe_batch_transfer_from");
  assert(status == "1");
  // TODO Can not do calculation on balance
  assert(erc1155.balance_of(myaddress1, "1") ==
         u256("999999999999999999999999800"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "2") == u256("0"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "3") == u256("999999700"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(myaddress1, "4") == u256("999999600"));
  begin = measure_time(begin, "erc1155.balance_of");

  test_uint();
  test_approval();
}

void test_approval() {
  timepoint begin = std::chrono::high_resolution_clock::now();
  String mycronosrpc = getEnv("MYCRONOSRPC");
  char hdpath[100];
  int cointype = 60;
  int chainid = 777; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);

  String signer1_mnemonics = getEnv("SIGNER1_MNEMONIC");
  Box<Wallet> signer1_wallet = createWallet(signer1_mnemonics);
  begin = measure_time(begin, "createWallet");
  String signer1_address = signer1_wallet->get_eth_address(0);
  begin = measure_time(begin, "get_eth_address");
  Box<PrivateKey> signer1_privatekey = signer1_wallet->get_key(hdpath);
  begin = measure_time(begin, "get_key");

  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  Box<Wallet> signer2_wallet = createWallet(signer2_mnemonics);
  begin = measure_time(begin, "createWallet");
  String signer2_address = signer2_wallet->get_eth_address(0);
  begin = measure_time(begin, "get_eth_address");
  Box<PrivateKey> signer2_privatekey = signer2_wallet->get_key(hdpath);
  begin = measure_time(begin, "get_key");

  String validator1_mnemonics = getEnv("VALIDATOR1_MNEMONIC");
  Box<Wallet> validator1_wallet = createWallet(validator1_mnemonics);
  begin = measure_time(begin, "createWallet");
  String validator1_address = validator1_wallet->get_eth_address(0);
  begin = measure_time(begin, "get_eth_address");
  Box<PrivateKey> validator1_privatekey = validator1_wallet->get_key(hdpath);
  begin = measure_time(begin, "get_key");

  Erc20 erc20 = new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A",
                          mycronosrpc, chainid)
                    .legacy();
  begin = measure_time(begin, "new_erc20");

  // signer1 approve singer2 allowance
  erc20.interval(3000).approve(signer2_address, "1000", *signer1_privatekey);
  begin = measure_time(begin, "erc20.approve");
  String allowance = erc20.allowance(signer1_address, signer2_address);
  begin = measure_time(begin, "erc20.allowance");
  assert(allowance == "1000");
  // transfer from signer1 to validator1 using the allowance mechanism
  erc20.transfer_from(signer1_address, validator1_address, "100",
                      *signer2_privatekey);
  begin = measure_time(begin, "erc20.transfer_from");
  allowance = erc20.allowance(signer1_address, signer2_address);
  begin = measure_time(begin, "erc20.allowance");
  assert(allowance == "900");

  Erc721 erc721 = new_erc721("0x2305f3980715c9D247455504080b41072De38aB9",
                             mycronosrpc, chainid)
                      .legacy();
  begin = measure_time(begin, "new_erc721");
  assert(erc721.balance_of(signer1_address) == u256("1"));
  begin = measure_time(begin, "balance_of");
  assert(erc721.get_approved("1") ==
         "0x0000000000000000000000000000000000000000");
  begin = measure_time(begin, "get_approved");
  // toggle set_approval_for_all
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 0);
  begin = measure_time(begin, "is_approved_for_all");
  erc721.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  begin = measure_time(begin, "erc721.set_approval_for_all");
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 1);
  begin = measure_time(begin, "erc721.is_approved_for_all");
  erc721.set_approval_for_all(signer2_address, false, *signer1_privatekey);
  begin = measure_time(begin, "erc721.set_approval_for_all");
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 0);
  begin = measure_time(begin, "erc721.is_approved_for_all");

  // signer1 approve singer2 to transfer erc721
  erc721.approve(signer2_address, "1", *signer1_privatekey);
  begin = measure_time(begin, "erc721.approve");
  assert(erc721.get_approved("1") == signer2_address);

  // safe transfer erc721 from signer1 to validator1
  String status = erc721
                      .safe_transfer_from(signer1_address, validator1_address,
                                          "1", *signer2_privatekey)
                      .status;
  begin = measure_time(begin, "erc721.safe_transfer_from");
  assert(status == "1");
  assert(erc721.balance_of(validator1_address) == u256("1"));
  begin = measure_time(begin, "erc721.balance_of");
  assert(erc721.owner_of("1") == validator1_address);
  begin = measure_time(begin, "erc721.owner_of");

  // validator1 set_approval_for_all for singer2 to transfer all assets
  assert(erc721.is_approved_for_all(validator1_address, signer2_address) == 0);
  begin = measure_time(begin, "erc721.is_approved_for_all");
  erc721.set_approval_for_all(signer2_address, true, *validator1_privatekey);
  begin = measure_time(begin, "erc721.set_approval_for_all");
  assert(erc721.is_approved_for_all(validator1_address, signer2_address) == 1);
  begin = measure_time(begin, "erc721.is_approved_for_all");
  // safe transfer erc721 from validator1 to signer1
  status = erc721
               .safe_transfer_from(validator1_address, signer1_address, "1",
                                   *signer2_privatekey)
               .status;
  begin = measure_time(begin, "erc721.safe_transfer_from");
  assert(status == "1");
  assert(erc721.balance_of(signer1_address) == u256("1"));
  begin = measure_time(begin, "erc721.balance_of");
  assert(erc721.owner_of("1") == signer1_address);
  begin = measure_time(begin, "erc721.owner_of");

  Erc1155 erc1155 = new_erc1155("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc",
                                mycronosrpc, chainid)
                        .legacy();
  begin = measure_time(begin, "erc1155.new_erc1155");
  // toggle set_approval_for_all
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 0);
  begin = measure_time(begin, "erc1155.is_approved_for_all");
  erc1155.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  begin = measure_time(begin, "erc1155.set_approval_for_all");
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 1);
  begin = measure_time(begin, "erc1155.is_approved_for_all");
  erc1155.set_approval_for_all(signer2_address, false, *signer1_privatekey);
  begin = measure_time(begin, "erc1155.set_approval_for_all");
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 0);
  begin = measure_time(begin, "erc1155.is_approved_for_all");
  // set approval for signer2
  erc1155.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  begin = measure_time(begin, "erc1155.set_approval_for_all");
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 1);
  begin = measure_time(begin, "erc1155.is_approved_for_all");
  Vec<String> token_ids, amounts;
  Vec<uint8_t> erc1155_data;
  token_ids.push_back("1");
  token_ids.push_back("3");
  token_ids.push_back("4");

  amounts.push_back("500");
  amounts.push_back("600");
  amounts.push_back("700");
  // and safe batch transfer from signer1 to validator1
  status = erc1155
               .safe_batch_transfer_from(signer1_address, validator1_address,
                                         token_ids, amounts, erc1155_data,
                                         *signer2_privatekey)
               .status;
  begin = measure_time(begin, "erc1155.safe_batch_transfer_from");
  assert(status == "1");
  // TODO Can not do calculation on balance
  assert(erc1155.balance_of(signer1_address, "1") ==
         u256("999999999999999999999999300"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer1_address, "2") == u256("0"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer1_address, "3") == u256("999999100"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer1_address, "4") == u256("999998900"));
  begin = measure_time(begin, "erc1155.balance_of");

  assert(erc1155.balance_of(signer2_address, "1") == u256("200"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer2_address, "2") == u256("1"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer2_address, "3") == u256("300"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(signer2_address, "4") == u256("400"));
  begin = measure_time(begin, "erc1155.balance_of");

  assert(erc1155.balance_of(validator1_address, "1") == u256("500"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(validator1_address, "2") == u256("0"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(validator1_address, "3") == u256("600"));
  begin = measure_time(begin, "erc1155.balance_of");
  assert(erc1155.balance_of(validator1_address, "4") == u256("700"));
  begin = measure_time(begin, "erc1155.balance_of");
}

void test_uint() {
  timepoint begin = std::chrono::high_resolution_clock::now();
  assert(u256("15") == u256("15", 10));
  begin = measure_time(begin, "u256");
  assert(u256("15") == u256("0xf", 16));
  begin = measure_time(begin, "u256");
  assert(u256("1000") == u256("100").add(u256("900")));
  begin = measure_time(begin, "u256.add");
  assert(u256("999999999999999999999999300") ==
         u256("1000000000000000000000000000").sub(u256("700")));
  begin = measure_time(begin, "u256.sub");
  assert(u256("199999999999999999980000200") ==
         u256("99999999999999999990000100").mul(u256("2")));
  begin = measure_time(begin, "u256.mul");
  assert(u256("1999999999999999999800002") ==
         u256("199999999999999999980000200").div(u256("100")));
  begin = measure_time(begin, "u256.div");
  assert(u256("800002") ==
         u256("1999999999999999999800002").rem(u256("1000000")));
  begin = measure_time(begin, "u256.rem");
  assert(u256("512003840009600008") == u256("800002").pow(u256("3")));
  begin = measure_time(begin, "u256.pow");
  assert(u256("512003840009600008").neg() ==
         u256_max_value().sub(u256("512003840009600007")));
  begin = measure_time(begin, "u256.neg + u256.sub");
}

void test_interval() {
  String mycronosrpc = getEnv("MYCRONOSRPC");
  char hdpath[100];
  int cointype = 60;
  int chainid = 777; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);

  String signer1_mnemonics = getEnv("SIGNER1_MNEMONIC");
  Box<Wallet> signer1_wallet = createWallet(signer1_mnemonics);
  String signer1_address = signer1_wallet->get_eth_address(0);
  Box<PrivateKey> signer1_privatekey = signer1_wallet->get_key(hdpath);

  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  Box<Wallet> signer2_wallet = createWallet(signer2_mnemonics);
  String signer2_address = signer2_wallet->get_eth_address(0);
  Box<PrivateKey> signer2_privatekey = signer2_wallet->get_key(hdpath);

  Erc20 erc20 = new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A",
                          mycronosrpc, chainid)
                    .legacy();

  // signer1 approve singer2 allowance
  timepoint begin = std::chrono::high_resolution_clock::now();
  erc20.interval(4000).approve(signer2_address, "1000", *signer1_privatekey);
  measure_time(begin, "erc20.approve polling 4000ms");
}

timepoint measure_time(timepoint t1, std::string label) {
  timepoint t2 = std::chrono::high_resolution_clock::now();

  // integral duration: requires duration_cast
  auto int_ms = std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1);

  // converting integral duration to integral duration of shorter divisible time
  // unit: no duration_cast needed
  auto int_usec =
      std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1);

  std::cout << "[" << label << "] " << int_ms.count() << " ms"
            << " (" << int_usec.count() << "us)" << std::endl;
  return t2;
}

void test_erc20_balance_of() {
  Erc20 erc20 = new_erc20("0xf0307093f23311FE6776a7742dB619EB3df62969",
                          "https://evm-dev-t3.cronos.org", 338)
                    .legacy();
  U256 balance = erc20.balance_of("0xf0307093f23311FE6776a7742dB619EB3df62969");
  cout << balance.to_string() << endl;
}

void test_erc20_name() {
  Erc20 erc20 = new_erc20("0xf0307093f23311FE6776a7742dB619EB3df62969",
                          "https://evm-dev-t3.cronos.org", 338)
                    .legacy();
  String name = erc20.name();
  assert(name == "USDC");
}

void test_erc20_symbol() {
  Erc20 erc20 = new_erc20("0xf0307093f23311FE6776a7742dB619EB3df62969",
                          "https://evm-dev-t3.cronos.org", 338)
                    .legacy();
  String symbol = erc20.symbol();
  assert(symbol == "USDC");
}

void test_erc20_decimals() {
  Erc20 erc20 = new_erc20("0xf0307093f23311FE6776a7742dB619EB3df62969",
                          "https://evm-dev-t3.cronos.org", 338)
                    .legacy();
  uint8_t decimals = erc20.decimals();
  assert(decimals == 6);
}

void test_erc20_total_supply() {
  Erc20 erc20 = new_erc20("0xf0307093f23311FE6776a7742dB619EB3df62969",
                          "https://evm-dev-t3.cronos.org", 338)
                    .legacy();
  U256 total_supply = erc20.total_supply();
  assert(total_supply == u256("100000000000000000000000000000000"));
}

// sample code for calling smart-contract
void test_dynamic_api_encode() {
  std::ifstream t("../../common/src/contract/erc721-abi.json");
  std::stringstream buffer;
  buffer << t.rdbuf();
  std::string json = buffer.str();

  String mymnemonics = getEnv("MYMNEMONICS");
  String mycronosrpc = getEnv("MYCRONOSRPC");
  String mycontract = getEnv("MYCONTRACT721");
  int mychainid = stoi(getEnv("MYCRONOSCHAINID").c_str());
  Box<Wallet> mywallet = createWallet(mymnemonics);

  String senderAddress = mywallet->get_eth_address(0);
  String receiverAddress = mywallet->get_eth_address(2);
  auto thisNonce = get_eth_nonce(senderAddress.c_str(), mycronosrpc);
  cout << "rpc=" << mycronosrpc << endl;
  std::string tokenid;
  std::cout << "Enter tokenid: ";
  std::cin >> tokenid;

  Box<EthContract> w = new_eth_contract(mycronosrpc, mycontract, json);

  char tmp[300];
  memset(tmp, 0, sizeof(tmp));
  sprintf(tmp,
          "[{\"Address\":{\"data\":\"%s\"}},{\"Address\":{\"data\":\"%s\"}},{"
          "\"Uint\":{\"data\":\"%d\"}}]",
          senderAddress.c_str(), receiverAddress.c_str(), stoi(tokenid));
  std::cout << tmp << std::endl;
  std::string paramsjson = tmp;
  Vec<uint8_t> data; // encoded
  data = w->encode("safeTransferFrom", paramsjson);
  cout << "data length=" << data.size() << endl;
  char hdpath[100];
  int cointype = 60;
  int chainid = mychainid; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);
  Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  EthTxInfoRaw eth_tx_info = new_eth_tx_info();
  eth_tx_info.to_address = mycontract;
  eth_tx_info.nonce = thisNonce;
  eth_tx_info.amount = "0";
  eth_tx_info.amount_unit = EthAmount::EthDecimal;
  eth_tx_info.data = data;
  eth_tx_info.gas_limit = "219400";
  eth_tx_info.gas_price = "100000000";
  eth_tx_info.gas_price_unit = EthAmount::WeiDecimal;

  Vec<uint8_t> signedtx =
      build_eth_signed_tx(eth_tx_info, chainid, true, *privatekey);
  CronosTransactionReceiptRaw receipt =
      broadcast_eth_signed_raw_tx(signedtx, mycronosrpc, 1000);
  String status = receipt.status;
  Vec<String> logs = receipt.logs;
  for (auto it = logs.begin(); it != logs.end(); ++it) {
    cout << *it << endl;
  }

  cout << "status: " << status << endl;
}

void test_dynamic_api_call() {

  std::ifstream t("../../common/src/contract/erc721-abi.json");
  std::stringstream buffer;
  buffer << t.rdbuf();
  std::string json = buffer.str();

  String mymnemonics = getEnv("MYMNEMONICS");
  String mycronosrpc = getEnv("MYCRONOSRPC");
  String mycontract = getEnv("MYCONTRACT721");

  Box<EthContract> mycontractcall =
      new_eth_contract(mycronosrpc, mycontract, json);

  std::string tokenid;
  std::cout << "Enter tokenid: ";
  std::cin >> tokenid;

  char tmp[300];
  memset(tmp, 0, sizeof(tmp));
  sprintf(tmp, "[{\"Uint\":{\"data\":\"%d\"}}]", stoi(tokenid));

  std::string response = mycontractcall->call("ownerOf", tmp).c_str();
  std::cout << "response: " << response << endl;
}

void test_dynamic_api_send() {
  std::ifstream t("../../common/src/contract/erc721-abi.json");
  std::stringstream buffer;
  buffer << t.rdbuf();
  std::string json = buffer.str();

  String mymnemonics = getEnv("MYMNEMONICS");
  String mycronosrpc = getEnv("MYCRONOSRPC");
  String mycontract = getEnv("MYCONTRACT721");
  int mychainid = stoi(getEnv("MYCRONOSCHAINID").c_str());
  Box<Wallet> mywallet = createWallet(mymnemonics);

  String senderAddress = mywallet->get_eth_address(0);
  String receiverAddress = mywallet->get_eth_address(2);
  auto thisNonce = get_eth_nonce(senderAddress.c_str(), mycronosrpc);
  cout << "rpc=" << mycronosrpc << endl;
  std::string tokenid;
  std::cout << "Enter tokenid: ";
  std::cin >> tokenid;

  char tmp[300];
  memset(tmp, 0, sizeof(tmp));
  sprintf(tmp,
          "[{\"Address\":{\"data\":\"%s\"}},{\"Address\":{\"data\":\"%s\"}},{"
          "\"Uint\":{\"data\":\"%d\"}}]",
          senderAddress.c_str(), receiverAddress.c_str(), stoi(tokenid));
  std::cout << tmp << std::endl;
  std::string paramsjson = tmp;
  char hdpath[100];
  int cointype = 60;
  int chainid = mychainid; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);
  Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  Box<EthContract> w =
      new_signing_eth_contract(mycronosrpc, mycontract, json, *privatekey);
  CronosTransactionReceiptRaw receipt = w->send("safeTransferFrom", paramsjson);

  String status = receipt.status;
  Vec<String> logs = receipt.logs;
  for (auto it = logs.begin(); it != logs.end(); ++it) {
    cout << *it << endl;
  }

  cout << "status: " << status << endl;
}

void test_cronos_testnet() {
  test_erc20_balance_of();
  test_erc20_name();
  test_erc20_symbol();
  test_erc20_decimals();
  test_erc20_total_supply();
}

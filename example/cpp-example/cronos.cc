#include "defi-wallet-core-cpp/src/contract.rs.h"
#include "defi-wallet-core-cpp/src/lib.rs.h"
#include "defi-wallet-core-cpp/src/uint.rs.h"
#include "rust/cxx.h"
#include <cassert>
#include <iostream>

void cronos_process();
void test_uint();
void test_approval();
using namespace std;
using namespace org::defi_wallet_core;
using namespace rust::cxxbridge1;

std::string getEnv(std::string key);

Box<Wallet> createWallet(String mymnemonics);

void cronos_process() {
  std::cout << "cronos process" << std::endl;
  String mymnemonics = getEnv("SIGNER1_MNEMONIC");
  String mycronosrpc = getEnv("MYCRONOSRPC");
  Box<Wallet> mywallet = createWallet(mymnemonics);
  String myaddress1 = mywallet->get_eth_address(0);
  String myaddress2 = mywallet->get_eth_address(1);
  auto nonce1 = get_eth_nonce(myaddress1.c_str(), mycronosrpc);
  char hdpath[100];
  int cointype = 60;
  int chainid = 777; // defined in cronos-devnet.yaml
  snprintf(hdpath, sizeof(hdpath), "m/44'/%d'/0'/0/0", cointype);
  Box<PrivateKey> privatekey = mywallet->get_key(hdpath);
  Vec<uint8_t> data;
  EthTxInfoRaw eth_tx_info = new_eth_tx_info();
  cout << myaddress2 << endl;
  eth_tx_info.to_address = myaddress2.c_str();
  eth_tx_info.nonce = nonce1;
  eth_tx_info.amount = "1";
  eth_tx_info.amount_unit = EthAmount::EthDecimal;
  rust::Vec<uint8_t> signedtx =
      build_eth_signed_tx(eth_tx_info, chainid, true, *privatekey);
  U256 balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance.to_string()
       << endl;
  String status = broadcast_eth_signed_raw_tx(signedtx, mycronosrpc).status;
  assert(status == "1");

  balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance.to_string()
       << endl;

  Erc20 erc20 = new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A",
                          mycronosrpc, chainid)
                    .legacy();
  assert(erc20.name() == "Gold");
  assert(erc20.symbol() == "GLD");
  assert(erc20.decimals() == 18);
  U256 erc20_total_supply = erc20.total_supply();
  assert(erc20_total_supply == u256("100000000000000000000000000"));
  U256 erc20_balance = erc20.balance_of(myaddress1);
  assert(erc20_balance == erc20_total_supply);

  Erc721 erc721 = new_erc721("0x2305f3980715c9D247455504080b41072De38aB9",
                             mycronosrpc, chainid)
                      .legacy();
  assert(erc721.name() == "GameItem");
  assert(erc721.symbol() == "ITM");
  assert(erc721.token_uri("1") == "https://game.example/item-id-8u5h2m.json");
  // cout << "Total Supply of ERC721=" << erc721.total_supply() << endl; // the
  // contract must support IERC721Enumerable
  assert(erc721.owner_of("1") == myaddress1);
  assert(erc721.balance_of(myaddress1) == u256("1"));

  Erc1155 erc1155 = new_erc1155("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc",
                                mycronosrpc, chainid)
                        .legacy();
  // To be improved in the contract, now all uri are the same
  assert(erc1155.uri("0") == "https://game.example/api/item/{id}.json");
  assert(erc1155.uri("1") == "https://game.example/api/item/{id}.json");
  assert(erc1155.uri("2") == "https://game.example/api/item/{id}.json");
  assert(erc1155.uri("3") == "https://game.example/api/item/{id}.json");
  assert(erc1155.uri("4") == "https://game.example/api/item/{id}.json");
  assert(erc1155.balance_of(myaddress1, "0") == u256("1000000000000000000"));
  assert(erc1155.balance_of(myaddress1, "1") ==
         u256("1000000000000000000000000000"));
  assert(erc1155.balance_of(myaddress1, "2") == u256("1"));
  assert(erc1155.balance_of(myaddress1, "3") == u256("1000000000"));
  assert(erc1155.balance_of(myaddress1, "4") == u256("1000000000"));

  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  Box<Wallet> signer2_wallet = createWallet(signer2_mnemonics);
  String signer2_address = signer2_wallet->get_eth_address(0);
  Box<PrivateKey> signer2_privatekey = signer2_wallet->get_key(hdpath);

  // transfer erc20 token from signer1 to signer2
  status = erc20.transfer(signer2_address, "100", *privatekey).status;
  assert(status == "1");
  assert(erc20.balance_of(myaddress1) == erc20_balance.sub(u256("100")));

  // transfer erc721 from signer1 to signer2
  status = erc721.transfer_from(myaddress1, signer2_address, "1", *privatekey)
               .status;
  assert(status == "1");
  assert(erc721.balance_of(myaddress1) == u256("0"));
  assert(erc721.owner_of("1") == signer2_address);

  // safe transfer erc721 from signer2 to signer1
  status = erc721
               .safe_transfer_from(signer2_address, myaddress1, "1",
                                   *signer2_privatekey)
               .status;
  assert(status == "1");
  assert(erc721.balance_of(myaddress1) == u256("1"));
  assert(erc721.owner_of("1") == myaddress1);

  // safe transfer erc1155 from signer1 to signer2
  rust::Vec<uint8_t> erc1155_data;
  status = erc1155
               .safe_transfer_from(myaddress1, signer2_address, "0", "150",
                                   erc1155_data, *privatekey)
               .status;
  assert(status == "1");
  assert(erc1155.balance_of(myaddress1, "0") == u256("999999999999999850"));

  // safe batch transfer erc1155 from signer1 to signer2
  rust::Vec<String> token_ids, amounts;
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
  assert(status == "1");
  // TODO Can not do calculation on balance
  assert(erc1155.balance_of(myaddress1, "1") ==
         u256("999999999999999999999999800"));
  assert(erc1155.balance_of(myaddress1, "2") == u256("0"));
  assert(erc1155.balance_of(myaddress1, "3") == u256("999999700"));
  assert(erc1155.balance_of(myaddress1, "4") == u256("999999600"));

  test_uint();
  test_approval();
}

void test_approval() {
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

  String validator1_mnemonics = getEnv("VALIDATOR1_MNEMONIC");
  Box<Wallet> validator1_wallet = createWallet(validator1_mnemonics);
  String validator1_address = validator1_wallet->get_eth_address(0);
  Box<PrivateKey> validator1_privatekey = validator1_wallet->get_key(hdpath);

  Erc20 erc20 = new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A",
                          mycronosrpc, chainid)
                    .legacy();

  // signer1 approve singer2 allowance
  erc20.approve(signer2_address, "1000", *signer1_privatekey);
  String allowance = erc20.allowance(signer1_address, signer2_address);
  assert(allowance == "1000");
  // transfer from signer1 to validator1 using the allowance mechanism
  erc20.transfer_from(signer1_address, validator1_address, "100",
                      *signer2_privatekey);
  allowance = erc20.allowance(signer1_address, signer2_address);
  assert(allowance == "900");

  Erc721 erc721 = new_erc721("0x2305f3980715c9D247455504080b41072De38aB9",
                             mycronosrpc, chainid)
                      .legacy();
  assert(erc721.balance_of(signer1_address) == u256("1"));
  assert(erc721.get_approved("1") ==
         "0x0000000000000000000000000000000000000000");
  // toggle set_approval_for_all
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 0);
  erc721.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 1);
  erc721.set_approval_for_all(signer2_address, false, *signer1_privatekey);
  assert(erc721.is_approved_for_all(signer1_address, signer2_address) == 0);

  // signer1 approve singer2 to transfer erc721
  erc721.approve(signer2_address, "1", *signer1_privatekey);
  assert(erc721.get_approved("1") == signer2_address);

  // safe transfer erc721 from signer1 to validator1
  String status = erc721
                      .safe_transfer_from(signer1_address, validator1_address,
                                          "1", *signer2_privatekey)
                      .status;
  assert(status == "1");
  assert(erc721.balance_of(validator1_address) == u256("1"));
  assert(erc721.owner_of("1") == validator1_address);

  // validator1 set_approval_for_all for singer2 to transfer all assets
  assert(erc721.is_approved_for_all(validator1_address, signer2_address) == 0);
  erc721.set_approval_for_all(signer2_address, true, *validator1_privatekey);
  assert(erc721.is_approved_for_all(validator1_address, signer2_address) == 1);
  // safe transfer erc721 from validator1 to signer1
  status = erc721
               .safe_transfer_from(validator1_address, signer1_address, "1",
                                   *signer2_privatekey)
               .status;
  assert(status == "1");
  assert(erc721.balance_of(signer1_address) == u256("1"));
  assert(erc721.owner_of("1") == signer1_address);

  Erc1155 erc1155 = new_erc1155("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc",
                                mycronosrpc, chainid)
                        .legacy();
  // toggle set_approval_for_all
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 0);
  erc1155.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 1);
  erc1155.set_approval_for_all(signer2_address, false, *signer1_privatekey);
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 0);
  // set approval for signer2
  erc1155.set_approval_for_all(signer2_address, true, *signer1_privatekey);
  assert(erc1155.is_approved_for_all(signer1_address, signer2_address) == 1);
  rust::Vec<String> token_ids, amounts;
  rust::Vec<uint8_t> erc1155_data;
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
  assert(status == "1");
  // TODO Can not do calculation on balance
  assert(erc1155.balance_of(signer1_address, "1") ==
         u256("999999999999999999999999300"));
  assert(erc1155.balance_of(signer1_address, "2") == u256("0"));
  assert(erc1155.balance_of(signer1_address, "3") == u256("999999100"));
  assert(erc1155.balance_of(signer1_address, "4") == u256("999998900"));

  assert(erc1155.balance_of(signer2_address, "1") == u256("200"));
  assert(erc1155.balance_of(signer2_address, "2") == u256("1"));
  assert(erc1155.balance_of(signer2_address, "3") == u256("300"));
  assert(erc1155.balance_of(signer2_address, "4") == u256("400"));

  assert(erc1155.balance_of(validator1_address, "1") == u256("500"));
  assert(erc1155.balance_of(validator1_address, "2") == u256("0"));
  assert(erc1155.balance_of(validator1_address, "3") == u256("600"));
  assert(erc1155.balance_of(validator1_address, "4") == u256("700"));
}

void test_uint() {
  assert(u256("15") == u256("15", 10));
  assert(u256("15") == u256("0xf", 16));
  assert(u256("1000") == u256("100").add(u256("900")));
  assert(u256("999999999999999999999999300") ==
         u256("1000000000000000000000000000").sub(u256("700")));
  assert(u256("199999999999999999980000200") ==
         u256("99999999999999999990000100").mul(u256("2")));
  assert(u256("1999999999999999999800002") ==
         u256("199999999999999999980000200").div(u256("100")));
  assert(u256("800002") ==
         u256("1999999999999999999800002").rem(u256("1000000")));
  assert(u256("512003840009600008") == u256("800002").pow(u256("3")));
  assert(u256("512003840009600008").neg() ==
         u256_max_value().sub(u256("512003840009600007")));
}

#include "defi-wallet-core-cpp/src/contract.rs.h"
#include "defi-wallet-core-cpp/src/lib.rs.h"
#include "rust/cxx.h"
#include <iostream>

void cronos_process();
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
  String balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance << endl;
  String txhash = broadcast_eth_signed_raw_tx(signedtx, mycronosrpc);
  cout << "txhash=" << txhash << endl;
  balance = get_eth_balance(myaddress1.c_str(), mycronosrpc);
  cout << "address=" << myaddress1.c_str() << " balance=" << balance << endl;

  Box<ContractBalance> erc20_details =
      erc20_balance("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A");
  String erc20_balance =
      get_contract_balance(myaddress1, *erc20_details, mycronosrpc);
  cout << "GOLD balance=" << erc20_balance.c_str() << endl;

  Box<ContractBalance> erc721_details =
      erc721_balance("0x2305f3980715c9D247455504080b41072De38aB9");
  String erc721_balance =
      get_contract_balance(myaddress1, *erc721_details, mycronosrpc);
  cout << "GameItem balance=" << erc721_balance.c_str() << endl;

  Box<ContractBalance> erc1155_details_0 =
      erc1155_balance("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", "0");
  String erc1155_balance_0 =
      get_contract_balance(myaddress1, *erc1155_details_0, mycronosrpc);
  cout << "Balance of GOLD=" << erc1155_balance_0.c_str() << endl;

  Box<ContractBalance> erc1155_details_1 =
      erc1155_balance("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", "1");
  String erc1155_balance_1 =
      get_contract_balance(myaddress1, *erc1155_details_1, mycronosrpc);
  cout << "Balance of SILVER=" << erc1155_balance_1.c_str() << endl;

  Box<ContractBalance> erc1155_details_2 =
      erc1155_balance("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", "2");
  String erc1155_balance_2 =
      get_contract_balance(myaddress1, *erc1155_details_2, mycronosrpc);
  cout << "Balance of THORS_HAMMER=" << erc1155_balance_2.c_str() << endl;

  Box<ContractBalance> erc1155_details_3 =
      erc1155_balance("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", "3");
  String erc1155_balance_3 =
      get_contract_balance(myaddress1, *erc1155_details_3, mycronosrpc);
  cout << "Balance of SWORD=" << erc1155_balance_3.c_str() << endl;

  Box<ContractBalance> erc1155_details_4 =
      erc1155_balance("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", "4");
  String erc1155_balance_4 =
      get_contract_balance(myaddress1, *erc1155_details_4, mycronosrpc);
  cout << "Balance of SHIELD=" << erc1155_balance_4.c_str() << endl;

  Box<ContractOwner> erc721_owner_detail =
      erc721_owner("0x2305f3980715c9D247455504080b41072De38aB9", "1");
  String erc721_owner = get_token_owner(*erc721_owner_detail, mycronosrpc);
  cout << "Owner of token=" << erc721_owner.c_str() << endl;

  Erc20 erc20 =
      new_erc20("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A", mycronosrpc);
  cout << "Name of ERC20=" << erc20.name() << endl;
  cout << "Symbol of ERC20=" << erc20.symbol() << endl;
  cout << "Decimals of ERC20=" << int(erc20.decimals()) << endl;

  Erc721 erc721 =
      new_erc721("0x2305f3980715c9D247455504080b41072De38aB9", mycronosrpc);
  cout << "Name of ERC721=" << erc721.name() << endl;
  cout << "Symbol of ERC721=" << erc721.symbol() << endl;
  cout << "Token URI of ERC721=" << erc721.token_uri("1") << endl;

  Erc1155 erc1155 =
      new_erc1155("0x939D7350c54228e4958e05b65512C4a5BB6A2ACc", mycronosrpc);
  cout << "URI of ERC1155, GOLD=" << erc1155.uri("0") << endl;
  cout << "URI of ERC1155, SILVER=" << erc1155.uri("1") << endl;
  cout << "URI of ERC1155, THORS_HAMMER=" << erc1155.uri("2") << endl;
  cout << "URI of ERC1155, SWORD=" << erc1155.uri("3") << endl;
  cout << "URI of ERC1155, SHIELD=" << erc1155.uri("4") << endl;

  String signer2_mnemonics = getEnv("SIGNER2_MNEMONIC");
  Box<Wallet> signer2_wallet = createWallet(signer2_mnemonics);
  String signer2_address = signer2_wallet->get_eth_address(0);

  // transfer erc20 token from signer1 to signer2
  String response = erc20.transfer("0x5003c1fcc043D2d81fF970266bf3fa6e8C5a1F3A", signer2_address,
                 "100", chainid, *privatekey);

  erc20_balance = get_contract_balance(myaddress1, *erc20_details, mycronosrpc);
  cout << "GOLD balance after trasnfer=" << erc20_balance.c_str() << endl;


}

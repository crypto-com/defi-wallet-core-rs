#include "defi-wallet-core-cpp/include/user.h"

#include <algorithm>
#include <chrono>
#include <functional>
#include <iostream>
#include <thread>

#include "defi-wallet-core-cpp/src/lib.rs.h"
#include "rust/cxx.h"
using namespace std;

// org::defi_wallet_core
namespace org {
namespace defi_wallet_core {

DefiWalletCoreClient::DefiWalletCoreClient() {
  ::rust::Box<::org::defi_wallet_core::CronosTx> cronos =
      org::defi_wallet_core::new_cronos_tx();
  _cronosTX = cronos.into_raw();
}

DefiWalletCoreClient::~DefiWalletCoreClient() { destroyCronosTX(); }

void DefiWalletCoreClient::destroyCronosTX() {
  if (_cronosTX != NULL) {
    // restored back
    rust::cxxbridge1::Box<org::defi_wallet_core::CronosTx> tmptx =
        rust::cxxbridge1::Box<org::defi_wallet_core::CronosTx>::from_raw(
            _cronosTX);
    // drop called
    _cronosTX = NULL;
  }
}

void DefiWalletCoreClient::setCallback(ClientCallback callback) {
  mycallback = callback;
}

void DefiWalletCoreClient::run() {
  _cronosTX->start_working();
  while (true) {
    try {
      ::org::defi_wallet_core::CronosTransactionReceiptRaw receipt =
          _cronosTX->get_broadcast_tx_blocking();

      mycallback(receipt.transaction_hash.c_str(), receipt.success,
                 receipt.message.c_str(), receipt.jobid.c_str());

    } catch (const rust::cxxbridge1::Error& e) {
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
  }  // end of while
}

void DefiWalletCoreClient::initialize() {}
void DefiWalletCoreClient::broadcast_eth_tx(string raw_tx, string web3api_url,
                                            string jobid) {
  rust::Vec<::std::uint8_t> signedtx;
  
  copy(raw_tx.begin(), raw_tx.end(), back_inserter(signedtx));  
  assert(signedtx.size() == raw_tx.size());

  rust::cxxbridge1::String mycronosrpc(web3api_url.c_str());

  _cronosTX->broadcast_eth_signed_raw_tx_async(signedtx, mycronosrpc, jobid);
}

}  // namespace defi_wallet_core
}  // namespace org

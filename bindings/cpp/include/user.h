#pragma once
#include <memory>
#include <string>
#include <functional>

typedef std::function<void(std::string info, bool success, std::string message,
                           std::string jobid)>
    ClientCallback;
namespace org {
namespace defi_wallet_core {
struct CronosTx;
class DefiWalletCoreClient {
 public:
  CronosTx *_cronosTX;
  ClientCallback mycallback;
  DefiWalletCoreClient();
  ~DefiWalletCoreClient();
  void setCallback(ClientCallback callback);
  void broadcast_eth_tx(std::string raw_tx, std::string web3api_url,
                        std::string jobid);
  void run();
  void initialize();
  void destroyCronosTX();
};

}  // namespace defi_wallet_core
}  // namespace org

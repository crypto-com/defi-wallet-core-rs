# DeFi Wallet Core: WalletConnect 1.0 implementation
This crate contains the WalletConnect 1.0 client implementation the could be used by dApps in integrations.

## WalletConnect 1.0
For protocol details, see the technical specification: https://docs.walletconnect.com/tech-spec

## Usage
See "examples/web3.rs". The WalletConnect client implements the [ethers middleware](https://docs.rs/ethers/latest/ethers/providers/struct.Provider.html),
so one can call the Web3 JSON-RPC API methods: https://docs.walletconnect.com/json-rpc-api-methods/ethereum
after the client is linked with the external wallet.

You can use https://test.walletconnect.org/ for testing (not for production).

## Implementation
The implementation code is largely based off the unfinished WalletConnect Rust Client: https://github.com/nlordell/walletconnect-rs
The following major changes were made:
- The websocket implementation (originally `ws`) was replaced with `tokio-tungstenite` for better portability and compatibility with the async ecosystem.
- The cryptographic implementation (originally using `openssl`) was replaced with the [RustCrypto](https://github.com/RustCrypto) implementations in pure Rust
(given they are used elsewhere in the codebase as well as in major dependencies).
- The Ethereum transport implementation (originally using `web3`) was replaced with the `ethers` which is used elsewhere in the codebase. The extensibility of `ethers` allowed more Web3 methods to be reused.
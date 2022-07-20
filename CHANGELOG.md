# Changelog
## [0.x.x] - unreleased
### Added
- Add denomination query support to UniFFI bindings

## [0.2.1] - 2022-07-18
### Added
- Add CMake Support
- Add env `CPP_EXAMPLE_PATH` for cpp integration test
- Add a basic generation parser for EIP681
- Add a Make command `install-uniffi-bindgen`

### Changed
- Replaced the GH action with manual `cargo clippy` call and removed the duplicate clippy steps/flow
- Replace openssl with rustls
- Change the cargo test executions to use `cargo llvm-cov`
- Improve cpp integration test and assert rust::cxxbridge1::Error
- Replace `grpc-web-client` with `tonic-web-wasm-client`

## [0.2.0] - 2022-06-21
### Added
- Add polling interval argument or function for setting the polling interval on event filters and pending transactions.
- Add `CosmosParser` to support parsing both Protobuf and Proto3 JSON mapping of standard Cosmos and `crypto.org` messages.
- Add Luna Classic special messages to `proto-build`.
- Add pagination parameter for nft query: collection, denoms, owner
- Add cpp bindings documentation for developers
- Add js package build script
- Add iOS and Android tests and interfaces
- Add abi contract binding
- Add uniffi-binding feature for EthAbiTokenBind
- Add support for Luna Classic message
- Add crate type: rlib

### Changed
- Replace `CosmosCoin` with `SingleCoin`.
- Upgrade ethers to support eip1559 transaction requests on Cronos mainnet
- Replace Github action cache with Rust smart cache.
- Upgrade python dependencies
- Improve cpp integration test to support play-cpp-sdk
- Fix Android CI build error

### Removed
- Delete duplicate C-Sytle functions for Cosmos SDK in wasm binding.

## [0.1.12] - 2022-04-25
### Added
- Implement wallet generating, recovering from mnemonic and basic signing functions.
- Implement the all Cosmos message encoding, signing and broadcasting based on crate [cosmos-rust](https://github.com/cosmos/cosmos-rust).
- Implement basic transfer, ERC-20, ERC-721 and ERC-1155 functions of Ethereum based on crate [ethers-rs](https://github.com/gakonst/ethers-rs).
- Implement Cosmos `signDirect` function to support Protobuf signing directly.
- Implement Ethereum `eth_sign`, `personal_sign` and `eth_sign_transaction`.
- Implement dynamic EIP-712 encoding and signing from a JSON string (controlled by a feature `abi-contract`).
- Support `wasm` and `C++` bindings for the all above features.
- Add basic integration-test (uncompleted) with Dev node of [chain-main](https://github.com/crypto-org-chain/chain-main) and [cronos](https://github.com/crypto-org-chain/cronos).
- Add example code in folder `example`.

\[Unreleased\]: https://github.com/crypto-com/defi-wallet-core-rs/compare/v0.1.12...HEAD
\[0.1.12\]: https://github.com/crypto-com/defi-wallet-core-rs/releases/tag/v0.1.12

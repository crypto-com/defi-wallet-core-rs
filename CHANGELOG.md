# Changelog

## [Unreleased]
### Added
- Delete duplicate C-Sytle functions for Cosmos SDK in wasm binding.
- Add polling interval argument or function for setting the polling interval on event filters and pending transactions.
- Add `CosmosParser` to support parsing both Protobuf and Proto3 JSON mapping of standard Cosmos and `crypto.org` messages.
- Add Terra special messages to `proto-build`.

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

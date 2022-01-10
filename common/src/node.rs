/// wrappers around Cosmos SDK REST API and Tendermint RPC
/// FIXME: switch to grpc when grpc-web works in CosmRS: https://github.com/cosmos/cosmos-rust/pull/157
mod cosmos_sdk;
/// wrappers around Web3 API + basic contract types
mod ethereum;
pub use cosmos_sdk::*;
pub use ethereum::*;

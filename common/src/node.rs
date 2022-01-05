/// wrappers around Cosmos SDK REST API and Tendermint RPC
/// FIXME: switch to grpc when grpc-web works in CosmRS: https://github.com/cosmos/cosmos-rust/pull/157
mod cosmos_sdk;

pub use cosmos_sdk::*;

#![allow(ambiguous_glob_reexports)]

/// wrappers around Cosmos SDK REST API and Tendermint RPC
/// FIXME: switch to grpc when grpc-web works in CosmRS: https://github.com/cosmos/cosmos-rust/pull/157
mod cosmos_sdk;
/// wrappers around Web3 API + basic contract types
pub mod ethereum;
/// wrappers around chainmain NFT grpc/grpc-web API
pub mod nft;
/// wasm binding related functions
mod wasm_binding;

mod error;
pub use cosmos_sdk::*;
pub use error::*;
pub use ethereum::*;
pub use nft::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_binding::*;

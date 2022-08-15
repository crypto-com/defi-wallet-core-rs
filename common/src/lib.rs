/// Eth contract types generated from ABI
pub mod contract;
/// interactions with remote node RPC / API (querying, broadcast etc.)
pub mod node;
/// transaction building etc.
pub mod transaction;
/// HD wallet-related functionality
mod wallet;

/// Login module: signing using EIP-4361 on Ethereum or ADR-036 on Cosmos SDK
#[cfg(feature = "login")]
mod login;

/// QR code module: encoding and decoding of EIP-681 strings
#[cfg(feature = "qr-code")]
mod qr_code;

/// Utility functions
mod utils;

// expose all proto related types (e.g. for uniffi)
pub use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
pub use defi_wallet_core_proto as proto;
pub use proto::chainmain::nft::v1::*;
pub use proto::luna_classic::wasm::v1beta1::*;

pub use cosmrs::{tx::Msg, AccountId, Coin};
pub use eyre::{Report as ErrorReport, Result};
pub use ibc_proto::ibc::core::client::v1::Height;

#[cfg(feature = "login")]
pub use login::*;
pub use node::*;
#[cfg(feature = "qr-code")]
pub use qr_code::EIP681Request;
pub use transaction::*;
pub use wallet::*;
#[cfg(feature = "uniffi-binding")]
uniffi_macros::include_scaffolding!("common");

#[macro_use]
mod macros;

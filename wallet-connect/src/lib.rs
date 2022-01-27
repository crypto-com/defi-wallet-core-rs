/// the definitions related to WalletConnect 1.0 client implementation
mod client;
/// the cryptography helpers for WalletConnect 1.0
mod crypto;
/// small utilities for hexadecimal operations
mod hex;
/// the WalletConnect 1.0 relevant payload definitions: https://docs.walletconnect.com/tech-spec#events--payloads
mod protocol;
/// helpers for serde
mod serialization;
/// utilities for the connection URI: https://docs.walletconnect.com/tech-spec#requesting-connection
mod uri;

pub use client::*;
pub use protocol::*;

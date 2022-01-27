//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use core::fmt;

use super::Topic;
use ethers::prelude::Address;
pub use ethers::prelude::TransactionRequest as Transaction;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::Url;

/// the metadata of the peer (client or wallet)
/// that could be presented in the UI
/// https://docs.walletconnect.com/tech-spec#session-request
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// description of the dApp/wallet software
    pub description: String,
    /// a link to its homepage
    pub url: Url,
    /// links to icons ot display in the UI
    #[serde(default)]
    pub icons: Vec<Url>,
    /// name of the dApp/wallet software
    pub name: String,
}

/// the wrapper type of the metadata
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PeerMetadata {
    /// correct metadata as per WalletConnect 1.0 protocol specs
    Strict(Metadata),
    /// some extra or missing fields
    Malformed(Value),
}

/// the request to start a session with an external wallet
/// https://docs.walletconnect.com/tech-spec#session-request
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequest {
    /// the preferred chain ID
    pub chain_id: Option<u64>,
    /// sender's client id
    pub peer_id: Topic,
    /// sender's client metadata
    pub peer_meta: Metadata,
}

/// the response to the session request
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionParams {
    /// if the wallet user approved the connection
    pub approved: bool,
    /// the wallet's addresses
    pub accounts: Vec<Address>,
    /// the chain where these addresses are expected to be used
    pub chain_id: u64,
    /// the receiver/wallet's ID
    pub peer_id: Topic,
    /// the receiver/wallet's metadata
    pub peer_meta: PeerMetadata,
}

/// when the wallet disconnects or changes some information
/// (new accounts, a different chain id...)
/// https://docs.walletconnect.com/tech-spec#session-update
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdate {
    /// if the wallet user approved the connection
    pub approved: bool,
    /// the wallet's addresses
    pub accounts: Vec<Address>,
    /// the chain where these addresses are expected to be used
    pub chain_id: u64,
}

fn is_zst<T>(_t: &T) -> bool {
    std::mem::size_of::<T>() == 0
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON-RPC request
/// (taken from ethers as it's not exported)
pub struct Request<'a, T> {
    id: u64,
    jsonrpc: &'a str,
    method: &'a str,
    #[serde(skip_serializing_if = "is_zst")]
    params: T,
}

impl<'a, T> Request<'a, T> {
    /// Creates a new JSON RPC request
    pub fn new(id: u64, method: &'a str, params: T) -> Self {
        Self {
            id,
            jsonrpc: "2.0",
            method,
            params,
        }
    }
}

/// A JSON-RPC response
/// (taken from ethers as it's not exported)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<T> {
    /// it should correspond to the request id.
    /// according to https://www.jsonrpc.org/specification
    /// it could be "String, Number, or NULL"
    /// but we only use numbers in requests.
    /// TODO: change to be more general as per the official JSON-RPC specs?
    pub(crate) id: u64,
    jsonrpc: String,
    /// the result of the request
    #[serde(flatten)]
    pub data: ResponseData<T>,
}

/// the result of the request
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseData<R> {
    /// something went wrong
    Error { error: JsonRpcError },
    /// the result of the request
    Success { result: R },
}

impl<R> ResponseData<R> {
    /// Consume response and return value
    pub fn into_result(self) -> Result<R, JsonRpcError> {
        match self {
            ResponseData::Success { result } => Ok(result),
            ResponseData::Error { error } => Err(error),
        }
    }
}

impl ResponseData<serde_json::Value> {
    /// Encode the error to json value if it is an error
    #[allow(dead_code)]
    pub fn into_value(self) -> serde_json::Result<serde_json::Value> {
        match self {
            ResponseData::Success { result } => Ok(result),
            ResponseData::Error { error } => serde_json::to_value(error),
        }
    }
}

/// A JSON-RPC 2.0 error
#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub struct JsonRpcError {
    /// The error code
    pub code: i64,
    /// The error message
    pub message: String,
    /// Additional data
    pub data: Option<Value>,
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(code: {}, message: {}, data: {:?})",
            self.code, self.message, self.data
        )
    }
}

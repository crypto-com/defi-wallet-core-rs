use super::error::RestError;
#[cfg(not(target_arch = "wasm32"))]
use cosmos_sdk_proto::cosmos::{
    bank::v1beta1::{query_client::QueryClient, Metadata, QueryDenomMetadataRequest},
    tx::v1beta1::{service_client::ServiceClient, SimulateRequest},
};
#[cfg(not(target_arch = "wasm32"))]
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tendermint_rpc::{
    endpoint::broadcast::{tx_async, tx_commit, tx_sync},
    request, response,
};

mod balance_query;

pub use balance_query::*;

/// The raw response from the account API
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum RawRpcAccountResponse {
    /// the account was found
    OkResponse {
        /// the account details
        account: RawRpcAccountStatus,
    },
    /// error response -- usually when the account doesn't exit on-chain yet
    ErrorResponse {
        /// https://github.com/cosmos/cosmos-sdk/blob/9566c99185ad5ae64a56884d924ee354f211e6dd/types/errors/errors.go
        code: i64,
        /// the reason for failure
        message: String,
        /// usually empty; ignored for now
        #[serde(skip)]
        details: Vec<String>,
    },
}

/// the raw account status data from the account API
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RawRpcAccountStatus {
    /// the protobuf type
    #[serde(rename = "@type")]
    pub account_type: String,
    /// the bech32 address
    pub address: String,
    /// the associated public key
    pub pub_key: Option<RawRpcPubKey>,
    /// the global account number
    #[serde_as(as = "DisplayFromStr")]
    pub account_number: u64,
    /// the sequence number / nonce
    #[serde_as(as = "DisplayFromStr")]
    pub sequence: u64,
}

/// the raw pubkey data returned from the account API
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RawRpcPubKey {
    /// the protobuf type
    #[serde(rename = "@type")]
    pub pub_key_type: String,
    /// the pubkey payload encoded in base64
    pub key: String,
}

/// the version of balance API
/// see the breaking change: https://github.com/cosmos/cosmos-sdk/releases/tag/v0.42.11
#[derive(Clone, Copy)]
pub enum BalanceApiVersion {
    /// before 0.42.11 or 0.44.4
    Old,
    /// after 0.42.11 or 0.44.4
    New,
}

impl From<u8> for BalanceApiVersion {
    fn from(version: u8) -> BalanceApiVersion {
        match version {
            0 => BalanceApiVersion::Old,
            _ => BalanceApiVersion::New,
        }
    }
}

fn get_accounts_url(api_url: &str, address: &str) -> String {
    format!("{}/cosmos/auth/v1beta1/accounts/{}", api_url, address)
}

/// return the account details (async for JS/WASM)
pub async fn get_account_details(
    api_url: &str,
    address: &str,
) -> Result<RawRpcAccountResponse, RestError> {
    let resp = reqwest::Client::new()
        .get(get_accounts_url(api_url, address))
        .send()
        .await
        .map_err(RestError::RequestError)?
        .json::<RawRpcAccountResponse>()
        .await
        .map_err(RestError::RequestError)?;
    Ok(resp)
}

/// return the account details (blocking for other platforms;
/// platform-guarded as JS/WASM doesn't support the reqwest blocking)
#[cfg(not(target_arch = "wasm32"))]
pub fn get_account_details_blocking(
    api_url: &str,
    address: &str,
) -> Result<RawRpcAccountResponse, RestError> {
    let resp = reqwest::blocking::get(get_accounts_url(api_url, address))
        .map_err(RestError::RequestError)?
        .json::<RawRpcAccountResponse>()
        .map_err(RestError::RequestError)?;
    Ok(resp)
}

/// given the gRPC endpoint and the raw signed transaction bytes,
/// it'll submit the transaction for simulating its execution and return the used gas.
#[cfg(not(target_arch = "wasm32"))]
pub fn simulate_blocking(grpc_url: &str, tx: Vec<u8>) -> Result<u64, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    let result = rt.block_on(async move {
        let mut client = ServiceClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = SimulateRequest {
            tx_bytes: tx,
            ..Default::default()
        };
        let res = client
            .simulate(request)
            .await
            .map_err(RestError::GRPCError)?;
        res.into_inner().gas_info.ok_or(RestError::MissingResult)
    })?;
    Ok(result.gas_used)
}

/// Metadata about a coin denomination
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct DenomMetadata {
    /// "base represents the base denom (should be the DenomUnit with exponent = 0)."
    pub base: String,
    /// "name defines the name of the token (eg: Cosmos Atom)"
    pub name: String,
    /// description of the denomination
    pub description: String,
    /// "display indicates the suggested denom that should be displayed in clients."
    pub display: String,
    /// "symbol is the token symbol usually shown on exchanges (eg: ATOM). This can be the same as the display."
    pub symbol: String,
    /// known unit measures with aliases, formatted in json
    pub denom_units: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Metadata> for DenomMetadata {
    fn from(md: Metadata) -> Self {
        let denom_units = format!(
            "[{}]",
            md.denom_units
                .iter()
                .map(|unit| {
                    let aliases = unit.aliases.iter().map(|x| format!("\"{}\"", x)).join(",");

                    format!(
                        "{{\"denom\":\"{}\",\"exponent\":{},\"aliases\":[{}]}}",
                        unit.denom, unit.exponent, aliases
                    )
                })
                .join(",")
        );
        Self {
            base: md.base,
            name: md.name,
            description: md.description,
            display: md.display,
            symbol: md.symbol,
            denom_units,
        }
    }
}

/// given the gRPC endpoint and the denomination,
/// it'll return the denomination metadata
#[cfg(not(target_arch = "wasm32"))]
fn get_denom_metadata_blocking(grpc_url: &str, denom: String) -> Result<DenomMetadata, RestError> {
    // TODO: pass-in runtime (constructed inside the client?)
    // as part of this refactoring: https://github.com/crypto-com/defi-wallet-core-rs/issues/511 ?
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    let result = rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryDenomMetadataRequest { denom };
        let res = client
            .denom_metadata(request)
            .await
            .map_err(RestError::GRPCError)?;
        res.into_inner().metadata.ok_or(RestError::MissingResult)
    })?;
    Ok(result.into())
}

/// broadcast the tx (async for JS/WASM)
pub async fn broadcast_tx_sync(
    tendermint_rpc_url: &str,
    raw_signed_tx: Vec<u8>,
) -> Result<response::Wrapper<tx_sync::Response>, RestError> {
    let request = request::Wrapper::new(tx_sync::Request { tx: raw_signed_tx });

    reqwest::Client::new()
        .post(tendermint_rpc_url)
        .json(&request)
        .send()
        .await
        .map_err(RestError::RequestError)?
        .json::<response::Wrapper<tx_sync::Response>>()
        .await
        .map_err(RestError::RequestError)
}

/// The choice for Tendermint JSON-RPC transaction broadcast endpoint
pub enum TxBroadcastMode {
    /// returns the checkTx result
    Sync,
    /// returns immediately
    Async,
    /// returns the checkTx + deliverTx results or times out
    /// (mainly for development)
    Commit,
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! broadcast_tx {
    ($mode:ident, $raw_signed_tx:expr, $tendermint_rpc_url:expr) => {{
        let request = request::Wrapper::new($mode::Request {
            tx: $raw_signed_tx.into(),
        });
        let rpc_result = reqwest::blocking::Client::new()
            .post($tendermint_rpc_url)
            .json(&request)
            .send()
            .map_err(RestError::RequestError)?
            .json::<response::Wrapper<$mode::Response>>()
            .map_err(RestError::RequestError)?
            .into_result()
            .map_err(|_e| RestError::MissingResult)?;

        Ok(rpc_result.into())
    }};
}

#[cfg(not(target_arch = "wasm32"))]
fn broadcast_tx_blocking(
    tendermint_rpc_url: &str,
    raw_signed_tx: Vec<u8>,
    mode: TxBroadcastMode,
) -> Result<TxBroadcastResult, RestError> {
    match mode {
        TxBroadcastMode::Sync => broadcast_tx!(tx_sync, raw_signed_tx, tendermint_rpc_url),
        TxBroadcastMode::Async => broadcast_tx!(tx_async, raw_signed_tx, tendermint_rpc_url),
        TxBroadcastMode::Commit => broadcast_tx!(tx_commit, raw_signed_tx, tendermint_rpc_url),
    }
}

/// a subset of `tx_sync::Response` for UniFFI
#[derive(serde::Serialize, Debug)]
pub struct TxBroadcastResult {
    /// tendermint transaction hash in hexadecimal
    pub tx_hash_hex: String,
    /// error code (0 if success)
    pub code: u32,
    /// possible error log
    pub log: String,
}

impl From<tx_sync::Response> for TxBroadcastResult {
    fn from(resp: tx_sync::Response) -> Self {
        TxBroadcastResult {
            code: resp.code.value(),
            log: resp.log.to_string(),
            tx_hash_hex: resp.hash.to_string(),
        }
    }
}

impl From<tx_async::Response> for TxBroadcastResult {
    fn from(resp: tx_async::Response) -> Self {
        TxBroadcastResult {
            code: resp.code.value(),
            log: resp.log.to_string(),
            tx_hash_hex: resp.hash.to_string(),
        }
    }
}

impl From<tx_commit::Response> for TxBroadcastResult {
    fn from(resp: tx_commit::Response) -> Self {
        TxBroadcastResult {
            code: resp.deliver_tx.code.value(),
            log: resp.deliver_tx.log.to_string(),
            tx_hash_hex: resp.hash.to_string(),
        }
    }
}

/// broadcast the tx (blocking for other platforms;
/// platform-guarded as JS/WASM doesn't support the reqwest blocking)
#[cfg(not(target_arch = "wasm32"))]
pub fn broadcast_tx_sync_blocking(
    tendermint_rpc_url: &str,
    raw_signed_tx: Vec<u8>,
) -> Result<TxBroadcastResult, RestError> {
    broadcast_tx_blocking(tendermint_rpc_url, raw_signed_tx, TxBroadcastMode::Sync)
}

/// the client facade for communication with a Cosmos SDK-based node
#[cfg(not(target_arch = "wasm32"))]
pub struct CosmosSDKClient {
    /// the Tendermint JSON-RPC (usually on 26657)
    tendermint_rpc_url: String,
    /// the Cosmos gRPC (usually on 9090)
    grpc_url: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl CosmosSDKClient {
    /// a new client using a set of URLs
    pub fn new(tendermint_rpc_url: String, grpc_url: String) -> Self {
        Self {
            tendermint_rpc_url,
            grpc_url,
        }
    }

    /// broadcast the tx (blocking)
    /// default mode is "sync"
    pub fn broadcast_tx(
        &self,
        raw_signed_tx: Vec<u8>,
        mode: Option<TxBroadcastMode>,
    ) -> Result<TxBroadcastResult, RestError> {
        let txmode = mode.unwrap_or(TxBroadcastMode::Sync);
        broadcast_tx_blocking(&self.tendermint_rpc_url, raw_signed_tx, txmode)
    }

    /// return the balance (blocking)
    pub fn get_account_balance(
        &self,
        address: &str,
        denom: &str,
    ) -> Result<RawRpcBalance, RestError> {
        get_account_balance_blocking(&self.grpc_url, address, denom)
    }

    /// return the account details (blocking)
    pub fn get_account_details(&self, address: &str) -> Result<RawRpcAccountResponse, RestError> {
        get_account_details_blocking(&self.grpc_url, address)
    }

    /// return the denomination metadata (blocking)
    pub fn get_denom_metadata(&self, denom: &str) -> Result<DenomMetadata, RestError> {
        get_denom_metadata_blocking(&self.grpc_url, denom.to_owned())
    }

    /// it'll submit the transaction for simulating its execution and return the used gas.
    /// (blocking)
    pub fn simulate(&self, raw_signed_tx: Vec<u8>) -> Result<u64, RestError> {
        simulate_blocking(&self.grpc_url, raw_signed_tx)
    }
}

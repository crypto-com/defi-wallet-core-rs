/// The main client state definitions
mod core;
/// The external options to create a client
mod options;
/// The wallet-connect session management
mod session;
/// The websocket connection management
mod socket;

use std::str::FromStr;

use async_trait::async_trait;
use ethers::prelude::{
    Address, FromErr, JsonRpcClient, Middleware, Provider, ProviderError, Signature,
};
use eyre::Context;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::protocol::Metadata;

use self::{
    core::{Connector, ConnectorError},
    options::Options,
};

/// The WalletConnect 1.0 client
/// (holds the middleware trait implementations for ethers)
/// FIXME: a way for persisting and recovering the client state
/// (a callback function or a path which the client can control?)
#[derive(Debug)]
pub struct Client {
    connection: Connector,
}

impl Client {
    /// Creates a new client from the provided metadata
    /// (and will connect to the bridge server according to the URI in metadata)
    pub async fn new(meta: impl Into<Metadata>) -> Result<Self, ConnectorError> {
        Client::with_options(Options::new(meta.into())).await
    }

    /// Creates a new client from the provided options
    /// (and will connect to the bridge server according to the URI in metadata)
    pub async fn with_options(options: Options) -> Result<Self, ConnectorError> {
        Ok(Client {
            connection: Connector::new(options).await?,
        })
    }

    /// This will return an existing session or create a new session.
    /// If successful, the returned value is the wallet's addresses and the chain ID.
    /// TODO: more specific error types than eyre
    pub async fn ensure_session(&mut self) -> Result<(Vec<Address>, u64), eyre::Error> {
        self.connection.ensure_session().await
    }

    /// Send a request to sign a message as per https://eips.ethereum.org/EIPS/eip-1271
    pub async fn personal_sign(
        &mut self,
        message: &str,
        address: &Address,
    ) -> Result<Signature, ClientError> {
        let sig_str: String = self
            .request(
                "personal_sign",
                vec![
                    message.to_string(),
                    format!("{:?}", address),
                    "".to_string(),
                ],
            )
            .await?;
        Signature::from_str(&sig_str)
            .context("failed to parse signature")
            .map_err(ClientError::Eyre)
    }
}

/// Error thrown when sending an HTTP request
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("{0}")]
    Eyre(#[from] eyre::Report),
    #[error("Deserialization Error: {err}. Response: {text}")]
    /// Serde JSON Error
    SerdeJson {
        err: serde_json::Error,
        text: String,
    },
}

impl From<ClientError> for ProviderError {
    fn from(src: ClientError) -> Self {
        ProviderError::JsonRpcClientError(Box::new(src))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl JsonRpcClient for Client {
    type Error = ClientError;

    /// Sends a POST request with the provided method and the params serialized as JSON
    /// over HTTP
    async fn request<T: Serialize + Send + Sync + std::fmt::Debug, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ClientError> {
        self.connection.request(method, params).await
    }
}

/// The wrapper struct for `ethers` middleware
/// TODO: override transaction-related middleware methods,
/// so that the client broadcasts the transaction (instead of the wallet)?
#[derive(Debug)]
pub struct WCMiddleware<M>(M);

impl WCMiddleware<Provider<Client>> {
    /// Creates a new wrapper for `ethers` middleware
    pub fn new(client: Client) -> Self {
        WCMiddleware(Provider::new(client))
    }
}

/// The wrapper error type for `ethers` middleware-related issues
#[derive(Error, Debug)]
pub enum WCError<M: Middleware> {
    #[error("{0}")]
    MiddlewareError(M::Error),
    #[error("client error: {0}")]
    ClientError(ClientError),
}

impl<M: Middleware> FromErr<M::Error> for WCError<M> {
    fn from(src: M::Error) -> WCError<M> {
        WCError::MiddlewareError(src)
    }
}

#[async_trait]
impl<M> Middleware for WCMiddleware<M>
where
    M: Middleware,
{
    type Error = WCError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.0
    }
}

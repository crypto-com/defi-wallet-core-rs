use super::error::RestError;
use crate::proto;
use crate::transaction::*;
use grpc_web_client::Client;
use proto::chainmain::nft::v1::{
    query_client::QueryClient, Denom, QueryDenomsRequest, QueryDenomsResponse,
};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Endpoint;

pub async fn get_query_denoms(grpc_url: &str) -> Result<Vec<Denom>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_url.to_owned()));
    let request = QueryDenomsRequest { pagination: None };
    let res = client
        .denoms(request)
        .await
        .map_err(|_err| RestError::GRPCError)?
        .into_inner();
    Ok(res.denoms)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_query_denoms_blocking(grpc_url: &str) -> anyhow::Result<Vec<Denom>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let channel = Endpoint::new(grpc_url.to_owned())?.connect().await?;
        let mut client = QueryClient::new(channel);

        let request = QueryDenomsRequest { pagination: None };

        let res = client.denoms(request).await?.into_inner();

        Ok(res.denoms)
    })
}

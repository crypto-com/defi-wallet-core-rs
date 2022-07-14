// FIXME:
// It seems to be a `cargo-clippy` issue of Rust `1.61.0`.
// https://github.com/influxdata/influxdb_iox/commit/b2279fae3984a29e73a7070d0b99ae24675eb606
#![allow(clippy::await_holding_lock)]

use super::error::RestError;
use crate::proto;
use proto::chainmain::nft::v1::{
    query_client::QueryClient, BaseNft, Collection, Denom, Owner, QueryCollectionRequest,
    QueryDenomByNameRequest, QueryDenomRequest, QueryDenomsRequest, QueryNftRequest,
    QueryOwnerRequest, QuerySupplyRequest,
};

use crate::PageRequest;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::RwLock;

pub struct Client {
    #[cfg(target_arch = "wasm32")]
    pub client: QueryClient<tonic_web_wasm_client::Client>,
    #[cfg(not(target_arch = "wasm32"))]
    // uniffi does not support mutable reference, that's why RwLock here
    pub client: RwLock<QueryClient<tonic::transport::Channel>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub rt: tokio::runtime::Runtime,
}

impl Client {
    #[cfg(target_arch = "wasm32")]
    pub fn new(grpc_web_url: String) -> Self {
        let client = QueryClient::new(tonic_web_wasm_client::Client::new(grpc_web_url));
        Self { client }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_blocking(grpc_url: String) -> Result<Self, RestError> {
        let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
        let client = rt.block_on(async move {
            let client = QueryClient::connect(grpc_url.to_owned())
                .await
                .map_err(RestError::GRPCTransportError)?;
            Ok(client)
        });
        Ok(Self {
            client: RwLock::new(client?),
            rt,
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// Supply queries the total supply of a given denom or owner
    pub async fn supply(&mut self, denom_id: String, owner: String) -> Result<u64, RestError> {
        let request = QuerySupplyRequest { denom_id, owner };
        let res = self
            .client
            .supply(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.amount)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Supply queries the total supply of a given denom or owner
    pub fn supply_blocking(&self, denom_id: String, owner: String) -> Result<u64, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QuerySupplyRequest { denom_id, owner };
            let res = (*client)
                .supply(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.amount)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// Owner queries the NFTs of the specified owner
    pub async fn owner(
        &mut self,
        denom_id: String,
        owner: String,
        pagination: Option<PageRequest>,
    ) -> Result<Option<Owner>, RestError> {
        let request = QueryOwnerRequest {
            denom_id,
            owner,
            pagination,
        };
        let res = self
            .client
            .owner(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.owner)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Owner queries the NFTs of the specified owner
    pub fn owner_blocking(
        &self,
        denom_id: String,
        owner: String,
        pagination: Option<PageRequest>,
    ) -> Result<Option<Owner>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryOwnerRequest {
                denom_id,
                owner,
                pagination,
            };
            let res = (*client)
                .owner(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.owner)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// Collection queries the NFTs of the specified denom
    pub async fn collection(
        &mut self,
        denom_id: String,
        pagination: Option<PageRequest>,
    ) -> Result<Option<Collection>, RestError> {
        let request = QueryCollectionRequest {
            denom_id,
            pagination,
        };
        let res = self
            .client
            .collection(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.collection)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Collection queries the NFTs of the specified denom
    pub fn collection_blocking(
        &self,
        denom_id: String,
        pagination: Option<PageRequest>,
    ) -> Result<Option<Collection>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryCollectionRequest {
                denom_id,
                pagination,
            };
            let res = (*client)
                .collection(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.collection)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// Denom queries the definition of a given denom
    pub async fn denom(&mut self, denom_id: String) -> Result<Option<Denom>, RestError> {
        let request = QueryDenomRequest { denom_id };
        let res = self
            .client
            .denom(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.denom)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Denom queries the definition of a given denom
    pub fn denom_blocking(&self, denom_id: String) -> Result<Option<Denom>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryDenomRequest { denom_id };
            let res = (*client)
                .denom(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.denom)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// DenomByName queries the definition of a given denom by name
    pub async fn denom_by_name(&mut self, denom_name: String) -> Result<Option<Denom>, RestError> {
        let request = QueryDenomByNameRequest { denom_name };
        let res = self
            .client
            .denom_by_name(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.denom)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// DenomByName queries the definition of a given denom by name
    pub fn denom_by_name_blocking(&self, denom_name: String) -> Result<Option<Denom>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryDenomByNameRequest { denom_name };
            let res = (*client)
                .denom_by_name(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.denom)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// Denoms queries all the denoms
    pub async fn denoms(
        &mut self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<Denom>, RestError> {
        let request = QueryDenomsRequest { pagination };
        let res = self
            .client
            .denoms(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.denoms)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Denoms queries all the denoms
    pub fn denoms_blocking(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<Denom>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryDenomsRequest { pagination };
            let res = (*client)
                .denoms(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.denoms)
        })
    }

    #[cfg(target_arch = "wasm32")]
    /// NFT queries the NFT for the given denom and token ID
    pub async fn nft(
        &mut self,
        denom_id: String,
        token_id: String,
    ) -> Result<Option<BaseNft>, RestError> {
        let request = QueryNftRequest { denom_id, token_id };
        let res = self
            .client
            .nft(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();
        Ok(res.nft)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// NFT queries the NFT for the given denom and token ID
    pub fn nft_blocking(
        &self,
        denom_id: String,
        token_id: String,
    ) -> Result<Option<BaseNft>, RestError> {
        self.rt.block_on(async move {
            let mut client = self.client.write().unwrap();
            let request = QueryNftRequest { denom_id, token_id };
            let res = (*client)
                .nft(request)
                .await
                .map_err(RestError::GRPCError)?
                .into_inner();
            Ok(res.nft)
        })
    }
}

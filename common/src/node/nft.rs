use super::error::RestError;
use crate::proto;
use grpc_web_client::Client;
use proto::chainmain::nft::v1::{
    query_client::QueryClient, BaseNft, Collection, Denom, Owner, QueryCollectionRequest,
    QueryDenomByNameRequest, QueryDenomRequest, QueryDenomsRequest, QueryNftRequest,
    QueryOwnerRequest, QuerySupplyRequest,
};

/// Supply queries the total supply of a given denom or owner
pub async fn query_supply(
    grpc_web_url: &str,
    denom_id: String,
    owner: String,
) -> Result<u64, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QuerySupplyRequest { denom_id, owner };
    let res = client
        .supply(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.amount)
}

#[cfg(not(target_arch = "wasm32"))]
/// Supply queries the total supply of a given denom or owner
pub fn query_supply_blocking(grpc_url: &str, denom_id: String, owner: String) -> Result<u64, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QuerySupplyRequest { denom_id, owner };

        let res = client
            .supply(request)
            .await
            .map_err(RestError::GRPCError)?
        .into_inner();

        Ok(res.amount)
    })
}

/// Owner queries the NFTs of the specified owner
pub async fn query_owner(
    grpc_web_url: &str,
    denom_id: String,
    owner: String,
) -> Result<Option<Owner>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryOwnerRequest {
        denom_id,
        owner,
        pagination: None,
    };
    let res = client
        .owner(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.owner)
}

#[cfg(not(target_arch = "wasm32"))]
/// Owner queries the NFTs of the specified owner
pub fn query_owner_blocking(grpc_url: &str, denom_id: String, owner: String) -> Result<Option<Owner>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryOwnerRequest {
            denom_id,
            owner,
            pagination: None,
        };

        let res = client
            .owner(request)
            .await
            .map_err(RestError::GRPCError)?
        .into_inner();

        Ok(res.owner)
    })
}

/// Collection queries the NFTs of the specified denom
pub async fn query_collection(
    grpc_web_url: &str,
    denom_id: String,
) -> Result<Option<Collection>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryCollectionRequest {
        denom_id,
        pagination: None,
    };
    let res = client
        .collection(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.collection)
}

#[cfg(not(target_arch = "wasm32"))]
/// Collection queries the NFTs of the specified denom
pub fn query_collection_blocking(grpc_url: &str, denom_id: String) -> Result<Option<Collection>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryCollectionRequest {
            denom_id,
            pagination: None,
        };

        let res = client
            .collection(request)
            .await
            .map_err(RestError::GRPCError)?
        .into_inner();

        Ok(res.collection)
    })
}

/// Denom queries the definition of a given denom
pub async fn query_denom(grpc_web_url: &str, denom_id: String) -> Result<Option<Denom>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryDenomRequest { denom_id };
    let res = client
        .denom(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.denom)
}

#[cfg(not(target_arch = "wasm32"))]
/// Denom queries the definition of a given denom
pub fn query_denom_blocking(grpc_url: &str, denom_id: String) -> Result<Option<Denom>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryDenomRequest { denom_id };

        let res = client
            .denom(request)
            .await
            .map_err(RestError::GRPCError)?
        .into_inner();

        Ok(res.denom)
    })
}

/// DenomByName queries the definition of a given denom by name
pub async fn query_denom_by_name(
    grpc_web_url: &str,
    denom_name: String,
) -> Result<Option<Denom>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryDenomByNameRequest { denom_name };
    let res = client
        .denom_by_name(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.denom)
}

#[cfg(not(target_arch = "wasm32"))]
/// DenomByName queries the definition of a given denom by name
pub fn query_denom_by_name_blocking(grpc_url: &str, denom_name: String) -> Result<Option<Denom>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryDenomByNameRequest { denom_name };

        let res = client
            .denom_by_name(request)
            .await
            .map_err(RestError::GRPCError)?
        .into_inner();

        Ok(res.denom)
    })
}

/// Denoms queries all the denoms
pub async fn query_denoms(grpc_web_url: &str) -> Result<Vec<Denom>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryDenomsRequest { pagination: None };
    let res = client
        .denoms(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.denoms)
}

#[cfg(not(target_arch = "wasm32"))]
/// Denoms queries all the denoms
pub fn query_denoms_blocking(grpc_url: &str) -> Result<Vec<Denom>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryDenomsRequest { pagination: None };

        let res = client
            .denoms(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();

        Ok(res.denoms)
    })
}

/// NFT queries the NFT for the given denom and token ID
pub async fn query_nft(
    grpc_web_url: &str,
    denom_id: String,
    token_id: String,
) -> Result<Option<BaseNft>, RestError> {
    let mut client = QueryClient::new(Client::new(grpc_web_url.to_owned()));
    let request = QueryNftRequest { denom_id, token_id };
    let res = client
        .nft(request)
        .await
        .map_err(RestError::GRPCError)?
        .into_inner();
    Ok(res.nft)
}

#[cfg(not(target_arch = "wasm32"))]
/// NFT queries the NFT for the given denom and token ID
pub fn query_nft_blocking(
    grpc_url: &str,
    denom_id: String,
    token_id: String,
) -> Result<Option<BaseNft>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let mut client = QueryClient::connect(grpc_url.to_owned())
            .await
            .map_err(RestError::GRPCTransportError)?;
        let request = QueryNftRequest { denom_id, token_id };

        let res = client
            .nft(request)
            .await
            .map_err(RestError::GRPCError)?
            .into_inner();

        Ok(res.nft)
    })
}
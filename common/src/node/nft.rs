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

pub enum Query<'a> {
    /// Supply queries the total supply of a given denom or owner
    Supply(&'a str), // denom_id
    /// Owner queries the NFTs of the specified owner
    Owner,
    /// Collection queries the NFTs of the specified denom
    Collection(&'a str), // denom_id
    /// Denom queries the definition of a given denom
    Denom(&'a str), // denom_id
    /// DenomByName queries the definition of a given denom by name
    DenomByName(&'a str), // denom_name
    /// Denoms queries all the denoms
    Denoms,
    /// NFT queries the NFT for the given denom and token ID
    NFT(&'a str, &'a str), // denom_id, token_id
}

impl<'a> Query<'a> {
    pub fn get_url(self, api_url: &str) -> String {
        match self {
            Self::Supply(denom_id) => {
                format!("{}/chainmain/nft/collections/{}/supply", api_url, denom_id)
            }
            Self::Owner => format!("{}/chainmain/nft/nfts", api_url),
            Self::Collection(denom_id) => {
                format!("{}/chainmain/nft/collections/{}", api_url, denom_id)
            }
            Self::Denom(denom_id) => format!("{}/chainmain/nft/denoms/{}", api_url, denom_id),
            Self::DenomByName(denom_name) => {
                format!("{}/chainmain/nft/denoms/name/{}", api_url, denom_name)
            }
            Self::Denoms => format!("{}/chainmain/nft/denoms", api_url),
            Self::NFT(denom_id, token_id) => {
                format!("{}/chainmain/nft/nfts/{}/{}", api_url, denom_id, token_id)
            }
        }
    }
}

// /// The raw balance data from the balance API
// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct RawQueryDenomsResponse {
//     pub denoms: Vec<Denom>,
//     pub pagination: RawPageResponse,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct Denom {
//     pub id: String,
//     pub name: String,
//     pub schema: String,
//     pub creator: String,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct RawPageResponse {
//     pub next_key: Option<RawRpcPubKey>,
//     pub total: String,
// }

// /// the raw pubkey data returned from the account API
// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct RawRpcPubKey {
//     /// the protobuf type
//     #[serde(rename = "@type")]
//     pub pub_key_type: String,
//     /// the pubkey payload encoded in base64
//     pub key: String,
// }

/// return the balance (async for JS/WASM)
///
/// When no NFT
// {
//     "denoms": [],
//     "pagination": {
//         "next_key": null,
//         "total": "0"
//     }
// }

// One nft:
// {
//   "denoms": [
//     {
//       "id": "testdenomid",
//       "name": "testdenomname",
//       "schema": "\n                    {\n                        \"title\":\"Asset Metadata\",\n                        \"type\":\"object\",\n                        \"properties\":{\n                            \"name\":{\n                                \"type\":\"string\",\n                                \"description\":\"testidentity\"\n                            },\n                            \"description\":{\n                                \"type\":\"string\",\n                                \"description\":\"testdescription\"\n                            },\n                            \"image\":{\n                                \"type\":\"string\",\n                                \"description\":\"testdescription\"\n                            }\n                        }\n                    }",
//       "creator": "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy"
//     }
//   ],
//   "pagination": {
//     "next_key": null,
//     "total": "1"
//   }
// }

// pub async fn get_query_client(grpc_addr: &str) -> Result<QueryClient<Client>> {
//     let mut url = grpc_addr.to_owned();

//     if url.ends_with('/') {
//         url.pop();
//     }

//     let grpc_client = Client::new(url);
//     Ok(QueryClient::new(grpc_client))
// }

// #[cfg(not(feature = "wasm"))]
// async fn get_query_client(grpc_addr: &str) -> Result<QueryClient<Channel>> {
//     QueryClient::new(grpc_addr.to_owned())
//         .await
//         .context("error when initializing grpc client")
// }

pub async fn get_query_denoms(grpc_url: &str) -> Result<Vec<Denom>, RestError> {
    // let client = grpc_web_client::Client::new(grpc_url.to_owned());
    // let request = QueryDenomsRequest { pagination: None };

    // let resp = reqwest::Client::new()
    //     .get(Query::Denoms.get_url(api_url))
    //     .send()
    //     .await
    //     .map_err(RestError::RequestError)?
    //     .json::<RawQueryDenomsResponse>()
    //     .await
    //     .map_err(RestError::RequestError)?;
    // Ok(resp.denoms)
    // let mut client: QueryClient<Client> = get_query_client(grpc_url).await.unwrap();
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
pub fn get_query_denoms_blocking(grpc_url: &str) -> Result<Vec<Denom>, RestError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| RestError::AsyncRuntimeError)?;
    rt.block_on(async move {
        let channel = Endpoint::new(grpc_url.to_owned())
            .map_err(|_err| RestError::GRPCError)?
            .connect()
            .await
            .map_err(|_err| RestError::GRPCError)?;
        let mut client = QueryClient::new(channel);

        let request = QueryDenomsRequest { pagination: None };

        let res = client
            .denoms(request)
            .await
            .map_err(|_err| RestError::GRPCError)?
            .into_inner();

        Ok(res.denoms)
    })

    // let channel = Endpoint::from_static(api_url).connect().await.map_err(RestError::TransportError)?;
    // let mut client = proto::chainmain::nft::v1::query_client::QueryClient::new(channel);
    // let request = proto::chainmain::nft::v1::QueryDenomsRequest {
    //     pagination: None,
    // };
    // Ok(resp)
}

// impl From<RawQueryDenomsResponse> for Result<JsValue, JsValue> {
//     fn from(res: RawQueryDenomsResponse) -> Self {
//         JsValue::from_serde(&res).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
//     }

// }

// impl Into<Result<JsValue, JsValue>> for RawQueryDenomsResponse {
//     fn into(self) -> Result<JsValue, JsValue> {
//         JsValue::from_serde(&self).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
//     }

// }

// impl From<Result<RawQueryDenomsResponse, RestError>> for Result<JsValue, JsValue> {
//     fn from(res: Result<RawQueryDenomsResponse, RestError>) -> Self {
//         let denoms = res.map_err(|e| JsValue::from_str(&format!("error: {}", e)))?;

//         Ok(JsValue::from_serde(&denoms)
//            .map_err(|e| JsValue::from_str(&format!("error: {}", e)))?)
//     }
// }

// /// return the balance (async for JS/WASM)
// pub async fn query_denom_by_name(
//     api_url: &str,
//     address: &str,
//     denom: &str,
//     version: BalanceApiVersion,
// ) -> Result<RawRpcBalance, RestError> {
//     let resp = reqwest::Client::new()
//         .get(get_balance_url(api_url, address, denom, version))
//         .send()
//         .await
//         .map_err(RestError::RequestError)?
//     .json::<BalanceResponse>()
//         .await
//         .map_err(RestError::RequestError)?;
//     Ok(resp.balance)
// }

// /// return the balance (async for JS/WASM)
// pub async fn query_nft_token(
//     api_url: &str,
//     address: &str,
//     denom: &str,
//     version: BalanceApiVersion,
// ) -> Result<RawRpcBalance, RestError> {
//     let resp = reqwest::Client::new()
//         .get(get_balance_url(api_url, address, denom, version))
//         .send()
//         .await
//         .map_err(RestError::RequestError)?
//     .json::<BalanceResponse>()
//         .await
//         .map_err(RestError::RequestError)?;
//     Ok(resp.balance)
// }

// /// return the balance (async for JS/WASM)
// pub async fn query_denoms(
//     api_url: &str,
//     address: &str,
//     denom: &str,
//     version: BalanceApiVersion,
// ) -> Result<RawRpcBalance, RestError> {
//     let resp = reqwest::Client::new()
//         .get(get_balance_url(api_url, address, denom, version))
//         .send()
//         .await
//         .map_err(RestError::RequestError)?
//     .json::<BalanceResponse>()
//         .await
//         .map_err(RestError::RequestError)?;
//     Ok(resp.balance)
// }

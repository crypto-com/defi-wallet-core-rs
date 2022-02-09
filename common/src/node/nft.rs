use super::error::RestError;
use serde::{Deserialize, Serialize};
// use wasm_bindgen::JsValue;

enum Query<'a> {
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
    fn get_url(self, api_url: &str) -> String {
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

/// The raw balance data from the balance API
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawNftDenomsResponse {
    pub denoms: Vec<Denom>,
    pub pagination: RawPagination,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Denom {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawPagination {
    pub next_key: Option<RawRpcPubKey>,
    pub total: String,
}

/// the raw pubkey data returned from the account API
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawRpcPubKey {
    /// the protobuf type
    #[serde(rename = "@type")]
    pub pub_key_type: String,
    /// the pubkey payload encoded in base64
    pub key: String,
}

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

pub async fn get_query_denoms(api_url: &str) -> Result<RawNftDenomsResponse, RestError> {
    let resp = reqwest::Client::new()
        .get(Query::Denoms.get_url(api_url))
        .send()
        .await
        .map_err(RestError::RequestError)?
        .json::<RawNftDenomsResponse>()
        .await
        .map_err(RestError::RequestError)?;
    Ok(resp)
}

// impl From<RawNftDenomsResponse> for Result<JsValue, JsValue> {
//     fn from(res: RawNftDenomsResponse) -> Self {
//         JsValue::from_serde(&res).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
//     }

// }

// impl Into<Result<JsValue, JsValue>> for RawNftDenomsResponse {
//     fn into(self) -> Result<JsValue, JsValue> {
//         JsValue::from_serde(&self).map_err(|e| JsValue::from_str(&format!("error: {}", e)))
//     }

// }

// impl From<Result<RawNftDenomsResponse, RestError>> for Result<JsValue, JsValue> {
//     fn from(res: Result<RawNftDenomsResponse, RestError>) -> Self {
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
